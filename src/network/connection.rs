use pnet::transport::{self, TransportChannelType};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::net::IpAddr;
use std::error::Error;

use crate::network::icmp::{encode_request_packet_from_fragment, encode_reply_packet_from_fragment};
use crate::chat::message::{MessageId}

// TODO tune buffer size
const BUFFER_SIZE: usize = 4096;

pub struct Connection {
    destination_ip: IpAddr,
    tx: transport::TransportSender,
    rx: transport::TransportReceiver,
    // Keep track of messages we've sent for retransmission purposes
    messages_inflight: HashMap<MessageId, Message>,
    // Buffer messages we've received until we have all fragments
    messages_received: HashMap<MessageId, Message>,
}

impl Connection {
    pub fn new(destination_ip: IpAddr) -> Result<Self, std::io::Error> {
        let protocol = IpNextHeaderProtocols::Icmp;
        let (tx, rx) = transport::transport_channel(BUFFER_SIZE, TransportChannelType::Layer3(protocol))?;
        Ok(Self {
            destination_ip,
            tx,
            rx,
            messages_inflight: HashMap::new(),
            messages_received: HashMap::new(),
        })
    }

    pub fn send_payload(&self, payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let message = Message::from_payload(payload);
        self.messages_inflight.insert(message.message_id, message);
        for fragment in &message.fragments {
            let packet = encode_request_packet_from_fragment(fragment)?;
            self.send_packet(packet)?;
        }
        Ok(())
    }

    pub fn listen(&self) -> Result<Message, Box<dyn Error>> {
        loop {
            let (packet, _) = self.rx.recv_from(BUFFER_SIZE)?;
            let ipv4_packet = Ipv4Packet::new(packet).ok_or(IcmpChatError::PacketError)?;
            let icmp_packet = IcmpPacket::new(ipv4_packet.payload()).ok_or(IcmpChatError::PacketError)?;

            match icmp_packet.get_icmp_type() {
                IcmpTypes::EchoReply => self.handle_icmp_packet(&icmp_packet, self.messages_inflight),
                IcmpTypes::EchoRequest => {
                    self.handle_icmp_packet(&icmp_packet, self.messages_received);
                    icmp_packet.set_icmp_type(IcmpTypes::EchoReply);
                    // Send back the same packet we received
                    self.send_packet(icmp_packet)?;
                },
                _ => continue,
            }
        }
    }

    fn send_packet(&self, packet: MutableEchoRequestPacket) -> Result<(), Box<dyn Error>> {
        self.tx.send_to(packet, self.destination_ip)?;
        Ok(())
    }

    fn handle_icmp_packet(&self, icmp_packet: &IcmpPacket, messages_map: &mut HashMap<MessageId, Message>) {
        if let (Some(message_id), Some(fragment)) = self.extract_from_packet(&icmp_packet) {
            if let Some(message) = messages_map.get_mut(&message_id) {
                message.add_fragment(fragment);
                if message.contains_all_fragments() {
                    println!("{}", message);
                }
            }
        }
    }

    fn extract_from_packet(&self, icmp_packet: &IcmpPacket) -> (Option<MessageId>, Option<Fragment>) {
        let message_id = icmp_packet.identifier();
        let fragment = Fragment::from_packet(icmp_packet);
        (Some(message_id), fragment)
    }
}