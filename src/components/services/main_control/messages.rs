use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Ack(Uuid), // 0
    Msg(Uuid, Vec<u8>) // 1
}

impl Message {
    pub fn get_uuid(&self) -> &Uuid {
        match self {
            Self::Ack(uuid) => uuid,
            Self::Msg(uuid, _) => uuid,
        }
    }

    // Convert to bytes
    pub fn serialize(msg: Message) -> Vec<u8> {
        let mut msg_buffer = vec![];

        match msg {
            Message::Ack(uuid) => {
                msg_buffer.push(0);
                let mut uuid_b = uuid.as_bytes().to_vec();
                msg_buffer.append(&mut uuid_b);
            },
            Message::Msg(uuid, mut msg) => {
                msg_buffer.push(1);
                let mut uuid_b = uuid.as_bytes().to_vec();
                msg_buffer.append(&mut uuid_b);
                msg_buffer.append(&mut msg);
            }
        }
        return msg_buffer;
    }

    // Convert from bytes
    pub fn deserialize(raw_msg: Vec<u8>) -> Message {
        match raw_msg[0] {
            0 => {
                let mut uuid_bytes: [u8; 16] = [0; 16];
                uuid_bytes[..].clone_from_slice(&raw_msg[1..17]);
                let uuid = Uuid::from_bytes(uuid_bytes);
                return Message::Ack(uuid);
            },
            _ => {
                let mut uuid_bytes: [u8; 16] = [0; 16];
                uuid_bytes[..].clone_from_slice(&raw_msg[1..17]);
                let uuid = Uuid::from_bytes(uuid_bytes);

                let msg = &raw_msg[17..];
                return Message::Msg(uuid, msg.to_vec());
            }
        }

    }

    pub fn new(msg: Vec<u8>) -> Message {
        return Message::Msg(Uuid::new_v4(), msg);
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn conv_ack_message() {
        let uuid_str = "936DA01F9ABD4d9d80C702AF85C822A8";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let message = Message::Ack(uuid);

        let serialized_message = Message::serialize(message.clone());
        let original_message = Message::deserialize(serialized_message);
        assert_eq!(message, original_message);
    }

    #[test]
    fn conv_msg_message() {
        let uuid_str = "936DA01F9ABD4d9d80C702AF85C822A8";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let msg: Vec<u8> = vec![1,2,3,4];
        let message = Message::Msg(uuid, msg.clone());

        let serialized_message = Message::serialize(message.clone());
        let original_message = Message::deserialize(serialized_message);
        assert_eq!(message, original_message);
    }
}
