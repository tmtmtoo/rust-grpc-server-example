#[derive(Debug, Clone, PartialEq)]
pub struct Message(String);

impl Message {
    pub fn new(prefix: &str, name: &str, emoji: &str) -> Self {
        Self(format!(
            "{emoji} {prefix} {name} {emoji}",
            emoji = emoji,
            prefix = prefix,
            name = name
        ))
    }
}

impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_format() {
        let msg = Message::new("hello", "foo", "😉");
        let actual = msg.as_ref();
        let expected = "😉 hello foo 😉";
        assert_eq!(actual, expected);
    }
}
