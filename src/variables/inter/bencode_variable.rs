use std::any::Any;
use std::fmt::{Debug, Display};
use std::io;

pub trait BencodeVariable: Display + Debug + ToBencode + FromBencode {

    fn parse<V>(&self) -> io::Result<V>
    where
        V: BencodeCast<Self>,
        Self: Sized,
    {
        V::cast(self)
    }

    fn upcast(self) -> Box<dyn BencodeVariable>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait BencodeCast<T>: Sized {

    fn cast(value: &T) -> io::Result<Self>;
}

pub trait ToBencode {

    fn to_bencode(&self) -> Vec<u8>;
}

pub trait FromBencode {

    fn from_bencode(buf: &[u8]) -> io::Result<Self> where Self: Sized {
        let (x, _) = Self::from_bencode_with_offset(buf)?;
        Ok(x)
    }

    fn from_bencode_with_offset(buf: &[u8]) -> io::Result<(Self, usize)> where Self: Sized;
}
