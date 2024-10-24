use std::any::Any;
use std::str::{from_utf8, FromStr};

use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BencodeNumber {
    n: Vec<u8>,
    s: usize
}

impl BencodeNumber {

    const TYPE: BencodeType = BencodeType::Number;

    pub fn parse<V>(&self) -> Result<V, String> where V: FromStr {
        let str = from_utf8(&self.n).map_err(|e| e.to_string())?;
        str.parse::<V>().map_err(|_| "Failed to parse number.".to_string())
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

impl_decodable_number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 isize f32 f64);

impl BencodeVariable for BencodeNumber {

    fn encode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::with_capacity(self.s);

        r.push(Self::TYPE.prefix());
        r.extend_from_slice(&self.n);
        r.push(Self::TYPE.suffix());
        r
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Result<Self, String> where Self: Sized {
        let type_ = BencodeType::type_by_prefix(buf[off]).map_err(|e| e.to_string())?;
        if type_ != Self::TYPE {
            return Err("Byte array is not a bencode number.".to_string());
        }

        let mut off = off+1;

        let mut c = [0u8; 32];
        let mut s = off;

        while buf[off] != Self::TYPE.suffix() {
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
