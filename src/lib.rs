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

    use crate::variables::bencode_object::{BencodeObject, ObjectOptions};
    use crate::variables::bencode_array::BencodeArray;
    use crate::variables::bencode_object::PutObject;
    use crate::variables::bencode_array::AddArray;
    use crate::variables::bencode_bytes::BencodeBytes;
    use crate::variables::inter::bencode_variable::{BencodeVariable, FromBencode, ToBencode};

    #[test]
    fn main() {
        let d  = "d1:v5:0.1.01:t6:create1:qd6:record1:a5:class2:in6:domain18:uncentralized.unet7:addressi2130706433e3:ttli300e11:cache_flushi1e5:locali1eee".as_bytes();
        let d = BencodeObject::from_bencode(d).unwrap();
        let mut x = bencode!({
            "name": "Edward",
            "t": "TEST",
            "b": [
                "a",
                12307123,
                {
                    "no": 123
                }
            ],
            "p": d
        });

        let z = x.remove("name".to_string()).unwrap();
        println!("{:?}", z.as_any().downcast_ref::<BencodeBytes>().unwrap().parse::<String>());

        println!("{:?}", String::from_utf8(x.to_bencode()).unwrap());
        println!("{}", x.to_string());
    }
}
