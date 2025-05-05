pub mod variables;
mod utils;

#[macro_export]
macro_rules! bencode {
    ({ $($key:tt : $value:tt),* $(,)? }) => {{
        let mut ben = BencodeObject::new();
        $(
            ben.put($key, bencode!($value));
        )*
        ben
    }};

    ([ $($elem:tt),* $(,)? ]) => {{
        let mut ben = BencodeArray::new();
        $(
            ben.push(bencode!($elem));
        )*
        ben
    }};

    ($val:ident) => {{
        $val
    }};

    ($val:expr) => {{
        Box::<dyn BencodeVariable>::from($val)
    }};
}


#[cfg(test)]
mod tests {

    use crate::variables::bencode_object::BencodeObject;
    use crate::variables::bencode_array::BencodeArray;
    use crate::variables::bencode_object::PutObject;
    use crate::variables::bencode_array::AddArray;
    use crate::variables::bencode_bytes::BencodeBytes;
    use crate::variables::bencode_number::BencodeNumber;
    use crate::variables::inter::bencode_variable::{BencodeVariable, FromBencode, ToBencode};

    #[test]
    fn object() {
        let a = bencode!({
            "a": "HELLO WORLD",
            "b": 100.2
        });
        let b = BencodeObject::from_bencode(&a.to_bencode()).unwrap();
        assert_eq!(a.as_any().downcast_ref::<BencodeObject>().unwrap(), &b);

        let a = b"d1:a11:HELLO WORLD1:bi100.2ee";
        let b = BencodeObject::from_bencode(a).unwrap();
        assert_eq!(a.to_vec(), b.to_bencode());
        println!("Object encoding and decoding passed.");
    }

    #[test]
    fn array() {
        let a = bencode!([
            "HELLO WORLD",
            100.2
        ]);
        let b = BencodeArray::from_bencode(&a.to_bencode()).unwrap();
        assert_eq!(a.as_any().downcast_ref::<BencodeArray>().unwrap(), &b);

        let a = b"l11:HELLO WORLDi100.2ee";
        let b = BencodeArray::from_bencode(a).unwrap();
        assert_eq!(a.to_vec(), b.to_bencode());
        println!("Array encoding and decoding passed.");
    }

    #[test]
    fn number() {
        let a = bencode!(100.2);
        let b = BencodeNumber::from_bencode(&a.to_bencode()).unwrap();
        assert_eq!(a.as_any().downcast_ref::<BencodeNumber>().unwrap(), &b);

        let a = b"i100.2e";
        let b = BencodeNumber::from_bencode(a).unwrap();
        assert_eq!(a.to_vec(), b.to_bencode());
        println!("Number encoding and decoding passed.");
    }

    #[test]
    fn bytes() {
        let a = bencode!("HELLO WORLD");
        let b = BencodeBytes::from_bencode(&a.to_bencode()).unwrap();
        assert_eq!(a.as_any().downcast_ref::<BencodeBytes>().unwrap(), &b);

        let a = b"11:HELLO WORLD";
        let b = BencodeBytes::from_bencode(a).unwrap();
        assert_eq!(a.to_vec(), b.to_bencode());
        println!("Byte encoding and decoding passed.");
    }
}
