use std::net;
use std::error::Error;

use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::MutablePacket;
use pnet::util;

// TODO tune
static IPV4_HEADER_LEN: usize = 21;
static ICMP_HEADER_LEN: usize = 8;
static ICMP_PAYLOAD_LEN: usize = 32;

fn create_ipv4_packet<'a>(
    buffer_ip: &'a mut [u8],
    buffer_icmp: &'a mut [u8],
    dest: net::Ipv4Addr,
) -> Result<MutableIpv4Packet<'a>, Box<dyn Error>> {
    let mut ipv4_packet = MutableIpv4Packet::new(buffer_ip).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(IPV4_HEADER_LEN as u8);
    ipv4_packet.set_total_length((IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN) as u16);
    ipv4_packet.set_ttl(1000);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_destination(dest);

    let mut icmp_packet = MutableEchoRequestPacket::new(buffer_icmp).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    let checksum = util::checksum(&icmp_packet.packet_mut(), 2);
    icmp_packet.set_checksum(checksum);
    ipv4_packet.set_payload(icmp_packet.packet_mut());
    Ok(ipv4_packet)
}


pub fn encode_request_packet_from_fragment(fragment: &Fragment) -> Result<MutableEchoRequestPacket, Box<dyn Error>> {
    let mut buffer = vec![0; 8 + fragment.payload.len()];
    let mut packet = MutableEchoRequestPacket::new(&mut buffer).expect("Failed to create ICMP packet");

    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_identifier(fragment.message_id);
    packet.set_sequence_number(fragment.fragment_id);
    packet.set_payload(&fragment.payload);
    
    let checksum = checksum(&packet.packet(), 1);
    packet.set_checksum(checksum);

    Ok(packet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pnet::packet::icmp::IcmpPacket;
    
    #[test]
    fn test_encode_request_packet_from_fragment() {
        // Setup a sample Fragment
        let fragment = Fragment {
            message_id: 42,
            fragment_id: 7,
            payload: vec![1, 2, 3, 4, 5], // A sample payload, just for testing
        };

        // Try encoding
        let encoded_packet_result = encode_request_packet_from_fragment(&fragment);
        
        // Ensure encoding did not result in error
        assert!(encoded_packet_result.is_ok());
        
        let packet = encoded_packet_result.unwrap();
        
        // Validate ICMP Type
        assert_eq!(packet.get_icmp_type(), IcmpTypes::EchoRequest);
        
        // Validate identifier (message_id) and sequence number (fragment_id)
        assert_eq!(packet.get_identifier(), fragment.message_id);
        assert_eq!(packet.get_sequence_number(), fragment.fragment_id);
        
        // Validate payload
        assert_eq!(packet.payload(), fragment.payload.as_slice());

        // Validate checksum
        let expected_checksum = checksum(&packet.packet(), 1);
        assert_eq!(packet.get_checksum(), expected_checksum);
    }
}
