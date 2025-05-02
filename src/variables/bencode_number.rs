use std::any::Any;
use std::io;
use std::str::{from_utf8, FromStr};

use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::inter::bencode_types::BencodeTypes;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BencodeNumber {
    n: Vec<u8>,
    s: usize
}

impl BencodeNumber {

    pub fn parse<V>(&self) -> io::Result<V> where V: FromStr {
        let str = from_utf8(&self.n).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        str.parse::<V>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse number."))
    }
}

macro_rules! impl_decodable_number {
    ($($type:ty)*) => {
        $(
            impl From<$type> for BencodeNumber {

                fn from(value: $type) -> Self {
                    let value = value.to_string().into_bytes();
                    let s = value.len()+2;

                    Self {
                        n: value,
                        s
                    }
                    /*
                    let value = value.to_string();
                    let size = value.len()+2;

                    let bytes = value.as_ptr();
                    let len = value.len();
                    forget(value);

                    unsafe {
                        Self {
                            n: from_raw_parts(bytes, len),
                            s: size
                        }
                    }
                    */
                }
            }
        )*
    }
}

impl_decodable_number!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl BencodeVariable for BencodeNumber {

    fn get_type(&self) -> BencodeTypes {
        BencodeTypes::Number
    }

    fn encode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::with_capacity(self.s);

        r.push(BencodeTypes::Number.prefix());
        r.extend_from_slice(&self.n);
        r.push(BencodeTypes::Number.suffix());
        r
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> io::Result<Self> where Self: Sized {
        let type_ = BencodeTypes::type_by_prefix(buf[off])?;
        if type_ != BencodeTypes::Number {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Byte array is not a bencode number."));
        }

        let mut off = off+1;

        let mut c = [0u8; 32];
        let mut s = off;

        while buf[off] != BencodeTypes::Number.suffix() {
            c[off - s] = buf[off];
            off += 1;
        }

        let bytes = buf[s..off].to_vec();

        off += 1;
        s = off+1-s;

        Ok(Self {
            n: bytes,
            s
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn byte_size(&self) -> usize {
        self.s
    }

    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.n).to_string()
    }
}
