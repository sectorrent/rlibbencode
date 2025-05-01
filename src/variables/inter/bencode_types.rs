use std::io;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum BencodeTypes {
    Number,
    Object,
    Array,
    Bytes
}

impl BencodeTypes {

    pub fn is_prefix(&self, c: u8) -> bool {
        match self {
            Self::Bytes => c >= b'0' && c <= b'9',
            _ => c == self.prefix()
        }
    }

    pub fn prefix(&self) -> u8 {
        match self {
            Self::Number => b'i',
            Self::Array => b'l',
            Self::Object => b'd',
            _ => unimplemented!()
        }
    }

    pub fn suffix(&self) -> u8 {
        match self {
            Self::Number => b'e',
            Self::Array => b'e',
            Self::Object => b'e',
            _ => unimplemented!()
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self {
            Self::Bytes => b':',
            _ => unimplemented!()
        }
    }

    pub fn type_by_prefix(c: u8) -> io::Result<Self> {
        for btype in [Self::Number, Self::Array, Self::Object, Self::Bytes] {
            if btype.is_prefix(c) {
                return Ok(btype);
            }
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "Type prefix is not valid."))
    }
}
