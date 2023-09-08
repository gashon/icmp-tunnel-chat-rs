use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{checksum, IcmpTypes};
use pnet::packet::Packet;
use crate::errors::IcmpChatError;

pub struct IcmpService {
    destination: std::net::IpV4Addr,
}

impl IcmpService {
    pub fn new(destination: std::net::IpV4Addr) -> Self {
        Self { destination }
    }

    // Encode an array of bytes into the payload of an icmp packet.
    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, IcmpChatError> {
        let mut buffer = vec![0; 8 + payload.len()];
        let mut packet = MutableEchoRequestPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;
        packet.set_icmp_type(IcmpTypes::EchoRequest);
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