use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};

use pnet::packet::icmp::echo_request::EchoRequestPacket;
use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{
    icmp_packet_iter, transport_channel, TransportChannelType::Layer3, TransportReceiver,
    TransportSender,
};

use crate::chat::message::{Message, MessageId};
use crate::network::fragment::{Fragment, ICMP_BUFFER_SIZE};

// TODO tune buffer size
const CHANNEL_BUFFER_SIZE: usize = 4096;

pub struct Connection {
    destination_ip: Ipv4Addr,
    tx: TransportSender,
    rx: TransportReceiver,
    // Keep track of messages we've sent for retransmission purposes
    messages_inflight: HashMap<MessageId, Box<Message>>,
    // Buffer messages we've received until we have all fragments
    messages_received: HashMap<MessageId, Box<Message>>,
}

impl Connection {
    pub fn new(destination_ip: Ipv4Addr) -> Result<Self, Box<dyn Error>> {
        let protocol = Layer3(IpNextHeaderProtocols::Icmp);
        let (tx, rx) = transport_channel(CHANNEL_BUFFER_SIZE, protocol)
            .map_err(|err| format!("Error opening the channel: {}", err))?;

        Ok(Self {
            destination_ip,
            tx,
            rx,
            messages_inflight: HashMap::new(),
            messages_received: HashMap::new(),
        })
    }

    pub fn send_payload(&mut self, payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let message = Box::new(Message::from_payload(&payload));
        let message_id = message.message_id;

        self.messages_inflight.insert(message_id, message);
        // max payload size is 1472 bytes
        let mut buf: Vec<u8> = vec![0; 1472];

        // Borrow a reference to the inserted message
        if let Some(stored_message) = self.messages_inflight.get_mut(&message_id) {
            for fragment in &mut stored_message.fragments {
                let packet = fragment.as_mut().unwrap().to_ipv4_packet(
                    self.destination_ip,
                    IcmpTypes::EchoRequest,
                    &mut buf[..],
                )?;
                self.tx.send_to(packet, IpAddr::V4(self.destination_ip))?;
            }
        }

        Ok(())
    }

    pub fn listen(&mut self) -> Result<Message, Box<dyn Error>> {
        let mut rx_iterator = icmp_packet_iter(&mut self.rx);

        loop {
            if let Ok((icmp_packet, addr)) = rx_iterator.next() {
                if addr != IpAddr::V4(self.destination_ip) {
                    continue;
                }

                match icmp_packet.get_icmp_type() {
                    IcmpTypes::EchoReply => {
                        handle_icmp_packet(&icmp_packet, &mut self.messages_inflight)
                    }
                    IcmpTypes::EchoRequest => {
                        handle_icmp_packet(&icmp_packet, &mut self.messages_received);
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn handle_icmp_packet(
    icmp_packet: &IcmpPacket,
    messages_map: &mut HashMap<MessageId, Box<Message>>,
) {
    let fragment = Fragment::from_icmp_packet(icmp_packet)
        .expect("Failed to create fragment from ICMP packet");

    if let Some(message) = messages_map.get_mut(&fragment.message_id) {
        message.add_fragment(&fragment);
        if message.contains_all_fragments() {
            // return Some(message);
            println!("{}", message);
        }
    } else {
        // TODO handle out of order fragments
        // TODO naive = add num_fragments to each fragment
        let mut message = Box::new(Message::new(1000));
        message.add_fragment(&fragment);
        messages_map.insert(message.message_id, message);
    }
}
