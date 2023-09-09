use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use pnet::packet::icmp::{checksum, IcmpTypes};
use pnet::packet::Packet;

use crate::errors::IcmpChatError;
use crate::network::fragment::{Fragment}

// TODO tune
pub const MAX_PAYLOAD_SIZE: usize = 32;

pub fn encode_request_packet_from_fragment(fragment: &Fragment) -> Result<MutableEchoRequestPacket, IcmpChatError> {
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

// switch destination and source
// make echo reply
pub fn encode_echo_reply_from_packet(packet: &MutableEchoRequestPacket) -> Result<MutableEchoReplyPacket, IcmpChatError> {
    
    
    let mut buffer = vec![0; 8 + packet.payload().len()];
    let mut reply_packet = MutableEchoReplyPacket::new(&mut buffer).ok_or(IcmpChatError::PacketError)?;

    packet.set_icmp_type(IcmpTypes::EchoReply);

    Ok(reply_packet)
}