use crate::codec::{BrinkEncoder, BrinkDecoder};

pub mod message;
pub mod codec;

#[derive(Debug)]
pub struct BrinkData {
    key: String,
    version: i32,
    blob: Vec<u8>,
}

impl BrinkEncoder for BrinkData {}

impl BrinkDecoder for BrinkData {
    fn decode(buffer: Vec<u8>) -> Self {
        unimplemented!()
    }
}