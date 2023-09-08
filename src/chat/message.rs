use crate::network::icmp::{MAX_PAYLOAD_SIZE, encode_request_packet_from_fragment, encode_reply_packet_from_fragment};

pub struct Message {
    pub message_id: u16,
    // fragment_id is the index of the fragment in the message.
    pub fragments: Vec<Fragment>,
}

impl Message {
    pub fn new(message_id: u16, num_fragments: u16) -> Self {
        Self { message_id, fragments: Vec::with_capacity(num_fragments) }
    }

    pub fn from_payload_with_message_id(&self, message_id: u16, payload: Vec<u8>) -> Self {
        let mut fragments = get_fragments_from_payload(&payload);

        Self { message_id, fragments }
    }

    pub fn add_fragment(&mut self, fragment: Fragment) {
        if fragment.fragment_id > self.fragments.len() {
            // TODO handle out of order fragments
        } 
        
        self.fragments[fragment.fragment_id] = fragment;
    }

    fn get_fragments_from_payload(payload: &Vec<u8>) -> Vec<Fragment> {
        let num_fragments = (payload.len() as f32 / MAX_PAYLOAD_SIZE as f32).ceil() as u16;
        let mut fragments = Vec::with_capacity(num_fragments);

        for fragment_id in 0..num_fragments {
            let start = (fragment_id * MAX_PAYLOAD_SIZE) as usize;
            let end = ((fragment_id + 1) * MAX_PAYLOAD_SIZE) as usize;

            let fragment_payload = payload[start..end].to_vec();
            let fragment = Fragment::new(fragment_id, message_id, fragment_payload);

            fragments.push(fragment);
        }

        fragments
    }

}