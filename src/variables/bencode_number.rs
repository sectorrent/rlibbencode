use std::any::Any;
use std::{fmt, io};
use std::fmt::Formatter;
use crate::variables::inter::bencode_types::BencodeTypes;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable, FromBencode, ToBencode};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BencodeNumber {
    value: Vec<u8>
}

impl BencodeVariable for BencodeNumber {

    fn get_type(&self) -> BencodeTypes {
        BencodeTypes::Number
    }

    fn upcast(self) -> Box<dyn BencodeVariable> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn BencodeVariable> {
        Box::new(self.clone())
    }
}

macro_rules! impl_bencode_number {
    ($($type:ty)*) => {
        $(
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

            impl From<$type> for Box<dyn BencodeVariable> {

                fn from(value: $type) -> Self {
                    Box::new(BencodeNumber {
                        value: value.to_string().as_bytes().to_vec()
                    })
                }
            }
        )*
    }
}

impl_bencode_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl ToBencode for BencodeNumber {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(BencodeTypes::Number.prefix());
        buf.extend_from_slice(&self.value);
        buf.push(BencodeTypes::Number.suffix());
        buf
    }
}

impl FromBencode for BencodeNumber {

    fn from_bencode_with_offset(buf: &[u8]) -> io::Result<(Self, usize)> {
        if !BencodeTypes::from_code(buf[0]).eq(&BencodeTypes::Number) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for number"));
        }

        let mut off = 1;
        while buf[off] != BencodeTypes::Number.suffix() {
            off += 1;
        }

        Ok((Self {
            value: buf[1..off].to_vec()
        }, off + 1))
    }
}

impl fmt::Display for BencodeNumber {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.value.clone()).unwrap())
    }
}
