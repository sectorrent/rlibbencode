use std::any::Any;
use std::str::from_utf8;

use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::inter::bencode_type::BencodeType;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BencodeBytes {
    b: Vec<u8>,
    s: usize
}

impl BencodeBytes {

    const TYPE: BencodeType = BencodeType::Bytes;

    pub fn as_bytes(&self) -> &[u8] {
        &self.b
    }

    pub fn as_str(&self) -> Result<&str, String> {
        from_utf8(&self.b).map_err(|e| e.to_string())
    }
}

impl<const N: usize> From<[u8; N]> for BencodeBytes {

    fn from(value: [u8; N]) -> Self {
        Self {
            b: value.to_vec(),
            s: value.len()+value.len().to_string().len()+1
        }
    }
}

impl From<Vec<u8>> for BencodeBytes {

    fn from(value: Vec<u8>) -> Self {
        let l = value.len();
        Self {
            b: value,
            s: l+l.to_string().len()+1
        }
    }
}

impl From<&str> for BencodeBytes {

    fn from(value: &str) -> Self {
        Self {
            b: value.as_bytes().to_vec(),
            s: value.len()+value.len().to_string().len()+1
        }
    }
}

impl From<String> for BencodeBytes {

    fn from(value: String) -> Self {
        let value = value.into_bytes();
        let s = value.len()+value.len().to_string().len()+1;
        Self {
            b: value,//from_raw_parts(bytes, len),
            s
        }
        /*
        let bytes = value.as_ptr();
        let len = value.len();
        forget(value);

        unsafe {
            let value = from_raw_parts(bytes, len);

            Self {
                b: value,//from_raw_parts(bytes, len),
                s: value.len()+value.len().to_string().len()+1
            }
        }*/
    }
}

impl BencodeVariable for BencodeBytes {

    fn encode(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::with_capacity(self.s);

        r.extend_from_slice(self.b.len().to_string().as_bytes());
        r.push(Self::TYPE.delimiter());
        r.extend_from_slice(&self.b);
        r
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Result<Self, String> where Self: Sized {
        let type_ = BencodeType::type_by_prefix(buf[off]).map_err(|e| e.to_string())?;
        if type_ != Self::TYPE {
            return Err("Byte array is not a bencode bytes / string.".to_string());
        }

        let mut off = off;
        let mut len_bytes = [0u8; 8];
        let mut s = off;

        while buf[off] != Self::TYPE.delimiter() {
            len_bytes[off - s] = buf[off];
            off += 1;
        }

        let length = len_bytes.iter().take(off - s).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        let bytes = buf[off + 1..off + 1 + length].to_vec();

        off += 1+length;
        s = off-s;

        Ok(Self {
            b: bytes,
            s
        })
    }
    /*
    fn encode(&self) -> &[u8] {
        let mut data = vec![0u8; self.s];

        let len_str = self.b.len().to_string();
        let len_bytes = len_str.as_bytes();

        let mut index = 0;
        for &byte in len_bytes {
            data[index] = byte;
            index += 1;
        }
        data[index] = Self::TYPE.delimiter();
        index += 1;

        for &byte in self.b.iter() {
            data[index] = byte;
            index += 1;
        }

        let ptr = data.as_ptr();
        let len = data.len();

        forget(data);

        unsafe {
            from_raw_parts(ptr, len)
        }
    }
    */

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
        String::from_utf8_lossy(&self.b).to_string()
    }
}
