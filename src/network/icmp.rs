use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use pnet::packet::icmp::{checksum, IcmpTypes};
use pnet::packet::Packet;

use crate::errors::IcmpChatError;
use crate::network::fragment::{Fragment}

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
    pub fn new(&self, destination: std::net::IpV4Addr) -> Self {
        Self { destination }
    }

    pub fn encode_request_packet_from_fragment(&self, fragment: &Fragment) -> Result<MutableEchoRequestPacket, IcmpChatError> {
        let mut buffer = vec![0; 8 + fragment.payload.len()];
        let mut packet = MutableEchoRequestPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;

        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_identifier(fragment.message_id);
        packet.set_sequence_number(fragment.fragment_id);
        packet.set_payload(&fragment.payload);

        let checksum = checksum(&packet.packet().to_immutable());
        packet.set_checksum(checksum);

        Ok(packet)
    }

    pub fn encode_reply_packet_from_fragment(&self, fragment: &Fragment) -> Result<MutableEchoRequestPacket, IcmpChatError> {
        let mut buffer = vec![0; 8 + fragment.payload.len()];
        let mut packet = MutableEchoReplyPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;

        packet.set_icmp_type(IcmpTypes::EchoReply);
        packet.set_identifier(fragment.message_id);
        packet.set_sequence_number(fragment.fragment_id);
        packet.set_payload(&fragment.payload);

        let checksum = checksum(&packet.packet().to_immutable());
        packet.set_checksum(checksum);

        Ok(packet)
    }

}