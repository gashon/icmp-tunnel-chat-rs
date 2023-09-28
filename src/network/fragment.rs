// Fragments are wrappers around ICMP packets
// they use the ICMP packet's sequence number to keep track of which fragment they are
// and the ICMP packet's identifier to keep track of which message they belong to
// they also contain the payload of the message

use std::error::Error;
use std::net::Ipv4Addr;

use pnet::packet::icmp::echo_reply::{EchoReplyPacket, MutableEchoReplyPacket};
use pnet::packet::icmp::echo_request::{EchoRequestPacket, MutableEchoRequestPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpType, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::MutablePacket;
use pnet::packet::Packet;
use pnet::util::checksum;

use crate::chat::message::MessageId;

// TODO tune
const IPV4_HEADER_LEN: usize = 21;
const ICMP_HEADER_LEN: usize = 8;
pub const ICMP_PAYLOAD_LEN: usize = 32;
pub const ICMP_BUFFER_SIZE: usize = IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN;

pub type FragmentId = u16;

#[derive(Clone)]
pub struct Fragment {
    // sequence number of the fragment
    pub fragment_id: FragmentId,
    // identifier of the message
    pub message_id: MessageId,
    // payload of the message
    pub payload: Vec<u8>,
}

impl Fragment {
    pub fn new(fragment_id: FragmentId, message_id: MessageId, payload: Vec<u8>) -> Self {
        Self {
            fragment_id,
            message_id,
            payload,
        }
    }

    pub fn from_icmp_request_packet(packet: &EchoRequestPacket) -> Result<Self, Box<dyn Error>> {
        let fragment_id = packet.get_sequence_number();
        let message_id = packet.get_identifier();
        let payload = packet.payload().to_vec();

        Ok(Self {
            fragment_id,
            message_id,
            payload,
        })
    }

    pub fn from_icmp_reply_packet(packet: &EchoReplyPacket) -> Result<Self, Box<dyn Error>> {
        let fragment_id = packet.get_sequence_number();
        let message_id = packet.get_identifier();
        let payload = packet.payload().to_vec();

        Ok(Self {
            fragment_id,
            message_id,
            payload,
        })
    }

    pub fn from_icmp_packet(packet: &IcmpPacket) -> Result<Self, Box<dyn Error>> {
        match packet.get_icmp_type() {
            IcmpTypes::EchoRequest => Self::from_icmp_request_packet(
                &EchoRequestPacket::new(packet.packet())
                    .ok_or("Failed to create ICMP request packet")?,
            ),
            IcmpTypes::EchoReply => Self::from_icmp_reply_packet(
                &EchoReplyPacket::new(packet.packet())
                    .ok_or("Failed to create ICMP reply packet")?,
            ),
            _ => panic!("Invalid ICMP type"),
        }
    }

    pub fn from_ipv4_packet(packet: &Ipv4Packet) -> Result<Self, Box<dyn Error>> {
        let icmp_packet =
            IcmpPacket::new(packet.payload()).ok_or("Failed to create ICMP packet")?;

        Self::from_icmp_packet(&icmp_packet)
    }

    pub fn to_icmp_request_packet(&mut self) -> Result<MutableEchoRequestPacket, Box<dyn Error>> {
        let mut icmp_packet = MutableEchoRequestPacket::new(&mut self.payload[..]).unwrap();

        icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
        icmp_packet.set_identifier(self.message_id);
        icmp_packet.set_sequence_number(self.fragment_id);
        let checksum = checksum(&icmp_packet.packet_mut(), 2);
        icmp_packet.set_checksum(checksum);

        Ok(icmp_packet)
    }

    pub fn to_icmp_reply_packet(&mut self) -> Result<MutableEchoReplyPacket, Box<dyn Error>> {
        let mut icmp_packet = MutableEchoReplyPacket::new(&mut self.payload[..]).unwrap();

        icmp_packet.set_icmp_type(IcmpTypes::EchoReply);
        icmp_packet.set_identifier(self.message_id);
        icmp_packet.set_sequence_number(self.fragment_id);
        let checksum = checksum(&icmp_packet.packet_mut(), 2);
        icmp_packet.set_checksum(checksum);

        Ok(icmp_packet)
    }

    pub fn to_ipv4_packet<'a>(
        &mut self,
        destination_ip: Ipv4Addr,
        icmp_type: IcmpType,
        buf: &'a mut [u8],
    ) -> Result<MutableIpv4Packet<'a>, Box<dyn Error>> {
        let mut ipv4_packet = MutableIpv4Packet::new(buf).unwrap();

        ipv4_packet.set_version(4);
        ipv4_packet.set_header_length(IPV4_HEADER_LEN as u8);
        ipv4_packet.set_total_length((IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN) as u16);
        ipv4_packet.set_ttl(60);
        ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
        ipv4_packet.set_destination(destination_ip);

        match icmp_type {
            IcmpTypes::EchoRequest => {
                let mut icmp_packet = self.to_icmp_request_packet()?;
                ipv4_packet.set_payload(icmp_packet.packet_mut());
                Ok(ipv4_packet)
            }
            IcmpTypes::EchoReply => {
                let mut icmp_packet = self.to_icmp_reply_packet()?;
                ipv4_packet.set_payload(icmp_packet.packet_mut());
                Ok(ipv4_packet)
            }
            _ => panic!("Invalid ICMP type"),
        }
    }
}
