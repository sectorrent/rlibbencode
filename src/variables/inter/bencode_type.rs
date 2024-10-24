#[derive(Debug, PartialEq)]
pub enum BencodeType {

    Number,
    Object,
    Array,
    Bytes
}

impl BencodeType {

    pub fn is_prefix(&self, c: u8) -> bool {
        match self {
            BencodeType::Bytes => c >= b'0' && c <= b'9',
            _ => c == self.prefix()
        }
    }

    pub fn prefix(&self) -> u8 {
        match self {
            BencodeType::Number => b'i',
            BencodeType::Array => b'l',
            BencodeType::Object => b'd',
            _ => unimplemented!()
        }
    }

    pub fn suffix(&self) -> u8 {
        match self {
            BencodeType::Number => b'e',
            BencodeType::Array => b'e',
            BencodeType::Object => b'e',
            _ => unimplemented!()
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self {
            BencodeType::Bytes => b':',
            _ => unimplemented!()
        }
    }

    pub fn type_by_prefix(c: u8) -> Result<Self, String> {
        for btype in [BencodeType::Number, BencodeType::Array, BencodeType::Object, BencodeType::Bytes] {
            if btype.is_prefix(c) {
                return Ok(btype);
            }
        }

        Err("Type prefix is not valid.".to_string())
    }
}
