use uuid::Uuid;

pub enum Message {
    Ack(Uuid),
    Msg(Uuid, Vec<u8>)
}

impl Message {
    pub fn serialize(msg: Message) -> Vec<u8> {
        // First byte signals if it is Ack / Msg

        // Next 16 octets are for uuid

        // Everything else is message
        return vec![];
    }

    pub fn deserialize(raw_msg: Vec<u8>) -> Message {
        // First byte signals if it is Ack / Msg

        // Next 16 octets are for uuid

        // Everything else is message
        return Ack(Uuid::new_v4());
    }
}
