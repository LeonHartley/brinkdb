pub trait BrinkEncoder {}

pub trait BrinkDecoder {
    fn decode(buffer: Vec<u8>) -> Self;
}