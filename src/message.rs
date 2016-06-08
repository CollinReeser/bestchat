use std::fmt;

pub struct ChatMessage {
    pub username: String,
    pub msg: String,
}

impl ChatMessage {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.username.clone().into_bytes());
        bytes.push(0);
        bytes.extend(&self.msg.clone().into_bytes());

        return bytes;
    }
}

impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}: {}", self.username, self.msg);
    }
}

trait ToChatMessage {
    fn bytes_to_message(&self)-> ChatMessage;
}

impl<'a> ToChatMessage for &'a Vec<u8> {
    fn bytes_to_message(&self) -> ChatMessage {
        let (username_bytes, msg_bytes) = self.split_at(
            self.iter().position(|&x| x == 0).unwrap()
        );

        return ChatMessage {
            username: String::from_utf8(username_bytes.to_owned()).unwrap(),
            msg: String::from_utf8(msg_bytes[1..].to_owned()).unwrap(),
        };
    }
}
