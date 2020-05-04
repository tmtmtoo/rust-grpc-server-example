use derive_more::*;
#[derive(AsRef, Debug, Clone, PartialEq)]
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
