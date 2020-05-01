use super::*;

pub trait Greet {
    fn hello(&self) -> Message;
}
