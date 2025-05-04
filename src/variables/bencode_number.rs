use std::any::Any;
use std::io;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable};
use crate::variables::inter::bencode_wrapper::{FromBencode, ToBencode};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BencodeNumber {
    value: Vec<u8>
}

impl BencodeVariable for BencodeNumber {

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

macro_rules! impl_bencode_number {
    ($($type:ty)*) => {
        $(
            impl ToBencode for $type {

                fn to_bencode(&self) -> Vec<u8> {
                    let mut buf = Vec::new();
                    buf.push(b'i');
                    buf.extend_from_slice(self.to_string().as_bytes());
                    buf.push(b'e');
                    buf
                }
            }

            impl FromBencode for $type {

                fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
                    if buf[0] != b'i' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for number"));
                    }

                    let mut off = 1;
                    while buf[off] != b'e' {
                        off += 1;
                    }

                    Ok((String::from_utf8(buf[1..off].to_vec())
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?.parse::<$type>()
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse number."))?, off + 1))
                }
            }

            impl From<$type> for BencodeNumber {

                fn from(value: $type) -> Self {
                    Self {
                        value: value.to_string().as_bytes().to_vec()
                    }
                }
            }

            impl BencodeCast<BencodeNumber> for $type {

                fn cast(value: &BencodeNumber) -> io::Result<Self> {
                    Ok(String::from_utf8(value.value.clone()).unwrap().parse::<$type>().unwrap())
                }
            }
        )*
    }
}

impl_bencode_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl ToBencode for BencodeNumber {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(b'i');
        buf.extend_from_slice(&self.value);
        buf.push(b'e');
        buf
    }
}

impl FromBencode for BencodeNumber {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if buf[0] != b'i' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for number"));
        }

        let mut off = 1;
        while buf[off] != b'e' {
            off += 1;
        }

        Ok((Self {
            value: buf[1..off].to_vec()
        }, off + 1))
    }
}
