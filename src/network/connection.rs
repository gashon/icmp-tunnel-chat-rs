use pnet::transport::{self, TransportChannelType};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::net::IpAddr;

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

        // Todo bind to random port
        // Todo bind to ip

        Self {destination_ip, tx, rx, messages: HashMap::new()}
    }

    pub fn send_payload(&self, payload: Vec<u8>) -> Result<(), Box<dyn std::error:Error>> {
        let message = Message::from_payload(payload);
        self.messages_inflight.insert(message.message_id, message);

        let mut fragments = message.fragments;
        let mut sent_fragments = Vec::with_capacity(fragments.len());

        while fragments.len() > 0 {
            let fragment = fragments.pop().unwrap();
            let packet = encode_request_packet_from_fragment(&fragment)?;

            self.tx.send_to(packet, self.destination_ip)?;
        }

        Ok(())
    }

    pub fn listen(&self) -> Result<Message, IcmpChatError> {

        loop { 
            let (packet, _) = self.rx.recv_from(BUFFER_SIZE)?;

            let ipv4_packet = Ipv4Packet::new(packet).ok_or(IcmpChatError::PacketError)?;
            let icmp_packet = IcmpPacket::new(ipv4_packet.payload()).ok_or(IcmpChatError::PacketError)?;

            let message_id = icmp_packet.identifier();
            let fragment = Fragment::from_packet(&icmp_packet)?;

            // Echo request is a fragment from another client
            // We need to buffer it until we have all fragments
            // Then we can print the message
            if icmp_packet.get_icmp_type() == IcmpTypes::EchoRequest {
                let message = self.messages_received.get_mut(&message_id).unwrap();

                message.add_fragment(fragment);

                // If we have all fragments, print the message
                if message.contains_all_fragments() {
                    println!("{}", message);
                }
            } 
            // Echo reply is a verification that our message was received
            // We need to keep track of which fragments have been received
            // If we have all fragments, we can stop retransmitting
            // TODO handle retransmission + checksum verification
            else if icmp_packet.get_icmp_type() == IcmpTypes::EchoReply {
                let message = self.messages_inflight.get_mut(&message_id).unwrap();

                let fragment_id = icmp_packet.sequence_number();

                // If we have all fragments, print the message
                if message.contains_all_fragments() {
                    println!("{}", message);
                }
            } 
        }
    }

}