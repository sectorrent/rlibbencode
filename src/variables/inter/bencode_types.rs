#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BencodeTypes {

    Object,
    Array,
    Number,
    Bytes
}
