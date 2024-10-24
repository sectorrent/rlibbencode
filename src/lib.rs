pub mod variables;
pub mod utils;

#[cfg(test)]
mod tests {

    use crate::variables::bencode_array::{AddArray, BencodeArray};
    use crate::variables::bencode_object::{BencodeObject, PutObject};
    use crate::variables::inter::bencode_variable::BencodeVariable;

    #[test]
    fn main() {
        let mut obj = BencodeObject::new();
        obj.put("b", "bar");
        obj.put("c", "far");
        obj.put("n", 100);
        obj.put("y", [ 0u8, 0u8, 0u8, 0u8, 0u8, 0u8 ]);

        let mut arr = BencodeArray::new();
        arr.add("n");
        arr.add(123.56);
        obj.put("array", arr);

        let mut obj2 = BencodeObject::new();
        obj2.put("z", "another one");
        obj.put("object", obj2);
        obj.get_object_mut("object").unwrap().put("n", "mutate");

        //z.deref_mut();
        //obj.put("blank", "blonk");


        let encoded = obj.encode();
        println!("EXPECTED: {} ACTUAL: {}", obj.byte_size(), encoded.len());
        println!("{:?}", encoded);

        println!("{}", obj.to_string());

        let decoded = BencodeObject::decode(&encoded).unwrap();
        println!("{}", decoded.to_string());
    }

    /*
    #[test]
    fn number() {
        let original = 100.67;
        let encoded = original.to_bencode();
        let decoded = f64::from_bencode(&encoded, &mut 0);

        assert_eq!(original, decoded);

        println!("Bencode Number Encoding & Decoding 100%");
    }

    #[test]
    fn bytes() {
        let original = "blank test".to_string();
        let encoded = original.to_bencode();
        let decoded = String::from_bencode(&encoded, &mut 0);

        assert_eq!(original, decoded);

        println!("Bencode String Encoding & Decoding 100%");
    }

    #[test]
    fn array() {
        let mut vec = Vec::new();
        vec.push("number 1");
        vec.push("num 2");
        let encoded = vec.to_bencode();
        let decoded = Vec::<String>::from_bencode(&encoded, &mut 0);

        assert_eq!(vec.len(), decoded.len());

        for i in 0..=decoded.len()-1 {
            assert_eq!(vec[i], decoded[i]);
        }

        println!("Bencode Array Encoding & Decoding 100%");
    }

    #[test]
    fn object() {
        let mut dic = HashMap::new();
        dic.insert("hello".to_string(), "123123".to_string());
        dic.insert("bloop".to_string(), "another test".to_string());
        let encoded = dic.to_bencode();
        let decoded = HashMap::<String, String>::from_bencode(&encoded, &mut 0);

        assert_eq!(dic.len(), decoded.len());

        for key in decoded.keys() {
            if dic.contains_key(key) {
                assert_eq!(dic.get(key).unwrap(), decoded.get(key).unwrap());
            } else {
                panic!("Key '{}' does not exist in both maps", key);
            }
        }

        println!("Bencode Object Encoding & Decoding 100%");
    }
    */
}
