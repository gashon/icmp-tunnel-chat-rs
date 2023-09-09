use pnet::transport::{self, TransportChannelType};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::net::IpAddr;

use crate::chat::message::{MessageId}

pub struct Connection {
    destination_ip: IpAddr,
    tx: transport::TransportSender,
    rx: transport::TransportReceiver,

    // Keep track of messages we've sent so we can reassemble them when we receive them.
    messages: HashMap<MessageId, Message>,
}

impl Connection {
    pub fn new(destination_ip: IpAddr) -> Result<Self, std::io::Error> {
        let protocol = IpNextHeaderProtocols::Icmp;
        // TODO tune buffer size
        let (tx, rx) = transport::transport_channel(4096, TransportChannelType::Layer3(protocol))?;

        // Todo bind to random port
        // Todo bind to ip

        Self {destination_ip, tx, rx, messages: HashMap::new()}
    }

    pub fn send_payload(&self, payload: Vec<u8>) -> Result<(), IcmpChatError> {
        let message = Message::from_payload(payload);

        let mut fragments = message.fragments;
        let mut sent_fragments = Vec::with_capacity(fragments.len());

        while fragments.len() > 0 {
            let fragment = fragments.pop().unwrap();
            let packet = encode_request_packet_from_fragment(&fragment)?;

            let mut socket = IcmpSocket::bind()?;
            socket.send_to(packet, self.destination)?;

            sent_fragments.push(fragment); 
        }

        Ok(())
    }

    pub fn listen(&self) -> Result<Message, IcmpChatError> {
        let mut socket = IcmpSocket::bind()?;
        let mut buffer = [0; 1024];

        loop {
            let (packet, _) = socket.recv_from(&mut buffer)?;
            let packet = IcmpPacket::new(packet)?;

            if packet.get_icmp_type() == IcmpTypes::EchoRequest {
                let fragment = Fragment::from_packet(&packet)?;
                let message_id = packet.identifier();

                let message = match self.messages.get(&fragment.message_id) {
                    Some(message) => message,
                    None => {
                        let message = Message::new(message_id, Vec::with_capacity(fragment.num_fragments));
                        self.messages.insert(message_id, message);
                        self.messages.get(&message_id).unwrap()
                    }
                };

                message.add_fragment(fragment);

                if message.contains_all_fragments() {
                    return Ok(message);
                }
            }
        }
    }

}