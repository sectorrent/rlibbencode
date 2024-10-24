use std::any::Any;

pub trait BencodeVariable {

    fn encode(&self) -> Vec<u8>;

    fn decode(buf: &[u8]) -> Result<Self, String> where Self: Sized {
        Self::decode_with_offset(buf, 0)
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Result<Self, String> where Self: Sized;//Self where Self: Sized;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn byte_size(&self) -> usize;

    fn to_string(&self) -> String;
}
