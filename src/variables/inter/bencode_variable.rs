use std::any::Any;
use std::io;
use crate::variables::inter::bencode_wrapper::{FromBencode, ToBencode};

pub trait BencodeVariable: ToBencode + FromBencode {

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
