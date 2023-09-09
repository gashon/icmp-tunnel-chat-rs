// Fragments are wrappers around ICMP packets
// they use the ICMP packet's sequence number to keep track of which fragment they are
// and the ICMP packet's identifier to keep track of which message they belong to
// they also contain the payload of the message

pub struct Fragment {
    // sequence number of the fragment
    pub fragment_id: u16,
    // identifier of the message
    pub message_id: u16,
    // payload of the message
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