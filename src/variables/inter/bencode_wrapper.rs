use std::io;

pub trait ToBencode {

    fn to_bencode(&self) -> Vec<u8>;
}

pub trait FromBencode {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> where Self: Sized;
}
