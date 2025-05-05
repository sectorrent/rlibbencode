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
            b'i' => Self::Number,
            b'l' => Self::Array,
            b'd' => Self::Object,
            b'0'..=b'9' => Self::Bytes,
            _ => unimplemented!()
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
}
