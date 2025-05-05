use std::any::Any;
use std::io;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::bencode_object::BencodeObject;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable, FromBencode, ToBencode};

pub trait AddArray<V> {

    fn push(&mut self, value: V);

    fn insert(&mut self, index: usize, value: V);
}

pub trait GetArrayCast<T> {

    fn get_cast<V: BencodeCast<T>>(&self, index: usize) -> Option<V>;
}

pub struct BencodeArray {
    value: Vec<Box<dyn BencodeVariable>>
}

impl BencodeArray {

    pub fn new() -> Self {
        Self {
            value: Vec::new()
        }
    }

    pub fn get<V: BencodeVariable + 'static>(&self, index: usize) -> Option<&V> {
        self.value
            .get(index)?
            .as_any()
            .downcast_ref::<V>()
    }

    pub fn get_mut<V: BencodeVariable + 'static>(&mut self, index: usize) -> Option<&mut V> {
        self.value
            .get_mut(index)?
            .as_any_mut()
            .downcast_mut::<V>()
    }

    fn remove(&mut self, index: usize) -> Box<dyn BencodeVariable> {
        self.value.remove(index)
    }
}

impl BencodeVariable for BencodeArray {

    fn upcast(self) -> Box<dyn BencodeVariable> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ToBencode for BencodeArray {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'l'];
        for e in &self.value {
            buf.extend(e.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl FromBencode for BencodeArray {

    fn from_bencode_with_offset(buf: &[u8], offset: &mut usize) -> io::Result<Self> {
        if buf[0] != b'l' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for object"));
        }

        let mut value = Vec::new();

        *offset = 1;
        while buf[*offset] != b'e' {
            value.push(match buf[*offset] {
                b'l' => BencodeArray::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                b'm' => BencodeObject::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                b'i' => BencodeNumber::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                _b @ b'0'..=b'9' => BencodeBytes::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                _ => unimplemented!()
            });
        }

        *offset += 1;

        Ok(Self {
            value
        })
    }
}

macro_rules! impl_bencode_add_array {
    ($(($_type:ty, $value:ty)),*) => {
        $(
            impl AddArray<$value> for BencodeArray {

                fn push(&mut self, value: $value) {
                    self.value.push(Box::new(<$_type>::from(value)));
                }

                fn insert(&mut self, index: usize, value: $value) {
                    self.value.insert(index, Box::new(<$_type>::from(value)));
                }
            }
        )*
    };
}

impl_bencode_add_array!(
    (BencodeNumber, u8),
    (BencodeNumber, u16),
    (BencodeNumber, u32),
    (BencodeNumber, u64),
    (BencodeNumber, u128),
    (BencodeNumber, usize),
    (BencodeNumber, i8),
    (BencodeNumber, i16),
    (BencodeNumber, i32),
    (BencodeNumber, i64),
    (BencodeNumber, i128),
    (BencodeNumber, isize),
    (BencodeNumber, f32),
    (BencodeNumber, f64),

    (BencodeBytes, String),
    (BencodeBytes, &str),
    (BencodeBytes, Vec<u8>)
);

impl AddArray<BencodeObject> for BencodeArray {

    fn push(&mut self, value: BencodeObject) {
        self.value.push(Box::new(value));
    }

    fn insert(&mut self, index: usize, value: BencodeObject) {
        self.value.insert(index, Box::new(value));
    }
}

impl AddArray<BencodeArray> for BencodeArray {

    fn push(&mut self, value: BencodeArray) {
        self.value.push(Box::new(value));
    }

    fn insert(&mut self, index: usize, value: BencodeArray) {
        self.value.insert(index, Box::new(value));
    }
}






impl AddArray<Box<dyn BencodeVariable>> for BencodeArray {

    fn push(&mut self, value: Box<dyn BencodeVariable>) {
        self.value.push(value);
    }

    fn insert(&mut self, index: usize, value: Box<dyn BencodeVariable>) {
        self.value.insert(index, value);
    }
}



impl GetArrayCast<BencodeNumber> for BencodeArray {

    fn get_cast<V: BencodeCast<BencodeNumber>>(&self, index: usize) -> Option<V> {
        self.value
            .get(index)?
            .as_any()
            .downcast_ref::<BencodeNumber>()?.parse::<V>().ok()
    }
}

impl GetArrayCast<BencodeBytes> for BencodeArray {

    fn get_cast<V: BencodeCast<BencodeBytes>>(&self, index: usize) -> Option<V> {
        self.value
            .get(index)?
            .as_any()
            .downcast_ref::<BencodeBytes>()?.parse::<V>().ok()
    }
}
