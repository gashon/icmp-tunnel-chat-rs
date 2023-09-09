use std::sync::Mutex;
use crate::network::icmp::{MAX_PAYLOAD_SIZE, encode_request_packet_from_fragment, encode_reply_packet_from_fragment};

// We need to keep track of which message ids are reserved so we don't reuse them
lazy_static::lazy_static! {
    static ref RESERVED_MESSAGE_IDS: Mutex<Vec<u16>> = Mutex::new(Vec::new());
}

pub type MessageId = u16;

pub struct Message {
    pub message_id: MessageId,
    // fragment_id is the index of the fragment in the message.
    pub fragments: Vec<Fragment>,
}

// TODO handle display
impl Message {
    pub fn new(num_fragments: u16) -> Self {
        let message_id = create_message_id();

        Self { message_id, fragments: Vec::with_capacity(num_fragments) }
    }

    pub fn from_payload(payload: &Vec<u8>) -> Self {
        let message_id = create_message_id();
        let mut fragments = get_fragments_from_payload(message_id, &payload);

        Self { message_id, fragments }
    }

    pub fn contains_all_fragments(&self) -> bool {
        self.fragments.len() == self.fragments.capacity()
    }

    pub fn add_fragment(&mut self, fragment: Fragment) {
        if fragment.fragment_id > self.fragments.len() {
            // TODO handle out of order fragments
        } 
        
        self.fragments[fragment.fragment_id] = fragment;
    }

}

fn get_fragments_from_payload(message_id: MessageId, payload: &Vec<u8>) -> Vec<Fragment> {
    let num_fragments = (payload.len() as f32 / MAX_PAYLOAD_SIZE as f32).ceil() as u16;
    let mut fragments = Vec::with_capacity(num_fragments);

    for fragment_id in 0..num_fragments {
        let start = (fragment_id * MAX_PAYLOAD_SIZE) as usize;
        let end = ((fragment_id + 1) * MAX_PAYLOAD_SIZE) as usize;

        let fragment_payload = payload[start..end].to_vec();
        let fragment = Fragment::new(fragment_id, message_id, fragment_payload);

        fragments[fragment_id] = fragment;
    }

    fragments
}

fn message_id_is_reserved(message_id: MessageId) -> bool {
    RESERVED_MESSAGE_IDS.lock().unwrap().contains(&message_id)
}

fn create_message_id() -> MessageId {
    let mut message_id = 0;

    let mut reserved_ids = RESERVED_MESSAGE_IDS.lock().unwrap();

    // TODO refactor to improve performance
    // TODO handle overflow
    while message_id_is_reserved(message_id) {
        message_id += 1;
    }

    reserved_ids.push(message_id);
    message_id
}