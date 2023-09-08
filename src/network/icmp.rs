use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{checksum, IcmpTypes};
use pnet::packet::Packet;
use crate::errors::IcmpChatError;

pub struct IcmpService {
    destination: std::net::IpV4Addr,
}

// Todo this service needs to manage the sequence number + identifier
// We should track the identifier values, and which are available to use.
// On connection, two clients will agree on whether they are even or odd.
// This service will then track the outstanding and available identifiers.
// -- we need methods to get the next available identifier, and to mark an identifier as available.
// * This service should e broken down further. We need functions to encode/create packets
// * This service is responsible for managing the state of the icmp connection.
// * This service should be responsible for sending and receiving packets.
// * This means tracking the sequence number, and the available identifiers.
// * message payloads should be parsable as fragments
// * What should 
impl IcmpService {
    pub fn new(destination: std::net::IpV4Addr) -> Self {
        Self { destination }
    }

    fn encode_echo_request(&self, payload: &[u8]) -> Result<MutableEchoRequestPacket, IcmpChatError> {
        let mut buffer = vec![0; 8 + payload.len()];
        let mut packet = MutableEchoRequestPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;
        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_identifier(0);
        packet.set_payload(payload);
        Ok(packet)
    }



    // Encode an array of bytes into the payload of an icmp packet.
    pub fn encode(&self, payload: &[u8], icmp_type: IcmpTypes) -> Result<Vec<u8>, IcmpChatError> {
        if data.len() > ICMP_PAYLOAD_LEN {
            return Err(IcmpChatError::PayloadSizeExceeded);
        }



        packet.set_sequence_number(0);
        let checksum = checksum(&packet.packet().to_immutable());
        packet.set_checksum(checksum);

        Ok(packet)


        let mut buffer = vec![0; 8 + payload.len()];
        let mut packet = MutableEchoRequestPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;
        packet.set_destination(self.destination);
        packet.set_icmp_type(icmp_type);
        packet.set_identifier(0);
        packet.set_sequence_number(0);
        packet.set_payload(payload);
        let checksum = checksum(&packet.to_immutable());
        packet.set_checksum(checksum);
        Ok(packet.packet().to_vec())
    }

    // Decode an array of bytes into the payload of an icmp packet.
    // Expect the packet to be an echo request.
    pub fn decode(&self, packet: &[u8]) -> Result<Vec<u8>, IcmpChatError> {
        let packet = MutableEchoRequestPacket::new(packet.to_owned()).ok_or(IcmpChatError::InvalidPacket)?;
        if packet.get_icmp_type() == IcmpTypes::EchoRequest {
            Ok(packet.payload().to_vec())
        } else {
            Err(IcmpChatError::InvalidPacketType)
        }
    }

}