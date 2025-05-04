use std::any::Any;
use std::io;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable};
use crate::variables::inter::bencode_wrapper::{FromBencode, ToBencode};

impl ToBencode for String {

    fn to_bencode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(self.len().to_string().as_bytes());
        encoded.push(b':');
        encoded.extend_from_slice(self.as_bytes());
        encoded
    }
}

impl FromBencode for String {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if !(buf[0] >= b'0' && buf[0] <= b'9') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for bytes"));
        }

        let mut off = 0;
        while buf[off] != b':' {
            off += 1;
        }

        let length = buf.iter().take(off).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        Ok((String::from_utf8(buf[off + 1..off + 1 + length].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?, length + off + 1))
    }
}

impl ToBencode for &str {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.push(b':');
        buf.extend_from_slice(self.as_bytes());
        buf
    }
}

impl ToBencode for &[u8] {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.push(b':');
        buf.extend_from_slice(self);
        buf
    }
}
/*
impl FromBencode for Vec<u8> {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if !(buf[0] >= b'0' && buf[0] <= b'9') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for bytes"));
        }

        let mut off = 0;
        while buf[off] != b':' {
            off += 1;
        }

        let length = buf.iter().take(off).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        Ok((buf[off + 1..off + 1 + length].to_vec(), length + off + 1))
    }
}
*/




#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BencodeBytes {
    value: Vec<u8>
}

impl BencodeVariable for BencodeBytes {

    fn upcast(self) -> Box<dyn BencodeVariable> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Vec<u8>> for BencodeBytes {

    fn from(value: Vec<u8>) -> Self {
        Self {
            value
        }
    }
}

impl From<&Vec<u8>> for BencodeBytes {

    fn from(value: &Vec<u8>) -> Self {
        Self {
            value: value.clone()
        }
    }
}

impl From<&str> for BencodeBytes {

    fn from(value: &str) -> Self {
        Self {
            value: value.as_bytes().to_vec()
        }
    }
}

impl From<String> for BencodeBytes {

    fn from(value: String) -> Self {
        Self {
            value: value.as_bytes().to_vec()
        }
    }
}

impl From<&String> for BencodeBytes {

    fn from(value: &String) -> Self {
        Self {
            value: value.as_bytes().to_vec()
        }
    }
}

impl<const N: usize> From<[u8; N]> for BencodeBytes {

    fn from(value: [u8; N]) -> Self {
        Self {
            value: value.to_vec()
        }
    }
}

impl From<&[u8]> for BencodeBytes {

    fn from(value: &[u8]) -> Self {
        Self {
            value: value.to_vec()
        }
    }
}

impl BencodeCast<BencodeBytes> for Vec<u8> {

    fn cast(value: &BencodeBytes) -> io::Result<Self> {
        Ok(value.value.clone())
    }
}

impl BencodeCast<BencodeBytes> for String {

    fn cast(value: &BencodeBytes) -> io::Result<Self> {
        Ok(String::from_utf8(value.value.clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
    }
}

impl FromBencode for BencodeBytes {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if !(buf[0] >= b'0' && buf[0] <= b'9') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for bytes"));
        }

        let mut off = 0;
        while buf[off] != b':' {
            off += 1;
        }

        let length = buf.iter().take(off).fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        Ok((Self {
            value: buf[off + 1..off + 1 + length].to_vec()
        }, length + off + 1))
    }
}

impl ToBencode for BencodeBytes {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.value.len().to_string().as_bytes());
        buf.push(b':');
        buf.extend_from_slice(&self.value);
        buf
    }
}
