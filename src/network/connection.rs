

pub struct Connection {
    destination: std::net::IpV4Addr,
    messages: HashMap<u16, Message>,
}

impl Connection {
    pub fn new(destination: std::net::IpV4Addr) -> Self {
        Self { destination }
    }

    pub fn send_payload(&self, payload: Vec<u8>) -> Result<(), IcmpChatError> {
        let message_id = self.get_message_id();
        let message = Message::new(message_id, payload);

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