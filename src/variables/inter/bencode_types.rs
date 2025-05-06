#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BencodeTypes {
    Object,
    Array,
    Number,
    Bytes
}

impl BencodeTypes {

    pub fn from_code(c: u8) -> Self {
        match c {
            b'l' => Self::Array,
            b'd' => Self::Object,
            b'i' => Self::Number,
            b'0'..=b'9' => Self::Bytes,
            _ => unimplemented!()
        }
    }

    pub fn prefix(&self) -> u8 {
        match self {
            Self::Array => b'l',
            Self::Object => b'd',
            Self::Number => b'i',
            _ => unimplemented!()
        }
    }

    pub fn suffix(&self) -> u8 {
        match self {
            Self::Array => b'e',
            Self::Object => b'e',
            Self::Number => b'e',
            _ => unimplemented!()
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self {
            Self::Bytes => b':',
            _ => unimplemented!()
        }
    }
}
