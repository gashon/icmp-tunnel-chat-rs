use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use pnet::packet::icmp::{checksum, IcmpTypes};
use pnet::packet::Packet;

use crate::errors::IcmpChatError;
use crate::network::fragment::{Fragment}

// TODO tune
pub const MAX_PAYLOAD_SIZE: usize = 32;

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