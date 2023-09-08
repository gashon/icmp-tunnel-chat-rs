// fragment.rs defines the fragment type encoded in ICMP requests and its methods.

pub struct Fragment {
    pub fragment_id: u16,
    pub message_id: u16,
    pub payload: Vec<u8>,
}

impl Fragment {

    pub fn new(fragment_id: u16, message_id: u16, payload: Vec<u8>) -> Self {
        Self { fragment_id, message_id, payload }
    }

    pub fn from_packet(packet: &IcmpPacket) -> Result<Self, IcmpChatError> {
        let payload = packet.payload().to_vec();
        let fragment_id = packet.sequence_number();
        let message_id = packet.identifier();

        Ok(Self { fragment_id, message_id, payload })
    }
}