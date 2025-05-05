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
        let d = BencodeObject::new();
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

        let z = x.remove("name".to_string()).unwrap();//.as_any().downcast_ref::<BencodeObject>().unwrap().to_bencode();
        println!("{:?}", z.as_any().downcast_ref::<BencodeBytes>().unwrap().parse::<String>());

        println!("{:?}", String::from_utf8(x.to_bencode()).unwrap());


        let d: Vec<u8>  = vec![109, 49, 58, 118, 53, 58, 48, 46, 49, 46, 48, 49, 58, 116, 54, 58, 99, 114, 101, 97, 116, 101, 49, 58, 113, 109, 54, 58, 114, 101, 99, 111, 114, 100, 49, 58, 97, 53, 58, 99, 108, 97, 115, 115, 50, 58, 105, 110, 54, 58, 100, 111, 109, 97, 105, 110, 49, 56, 58, 117, 110, 99, 101, 110, 116, 114, 97, 108, 105, 122, 101, 100, 46, 117, 110, 101, 116, 55, 58, 97, 100, 100, 114, 101, 115, 115, 105, 50, 49, 51, 48, 55, 48, 54, 52, 51, 51, 101, 51, 58, 116, 116, 108, 105, 51, 48, 48, 101, 49, 49, 58, 99, 97, 99, 104, 101, 95, 102, 108, 117, 115, 104, 105, 49, 101, 53, 58, 108, 111, 99, 97, 108, 105, 49, 101, 101, 101];
        let z = BencodeObject::from_bencode(&d).unwrap();
        println!("{:?}", z.to_bencode());

        /*
        let mut x = bencode!([
            "name",
            "t",
            {
                //"x": "V",
                //"awd": 213,
                //"z": [
                //    "adiawhd",
                //    1238
                //]
            }
        ]);

        //let f = bencode!("asdasd");
        //let f = bencode!(123123);

        let x = Box::<dyn BencodeVariable>::from(10);

        x.get_mut::<BencodeObject>(2).unwrap().put("no", "ajwdiajwhjdoaiwd");

        println!("{:?}", x.get::<BencodeObject>(2).unwrap().get_cast::<String>("x").unwrap());
        println!("{:?}", x.get_cast::<String>(1).unwrap());

        println!("{:?}", String::from_utf8(x.to_bencode()).unwrap());


        let z = x.get::<BencodeObject>(2).unwrap().get::<BencodeArray>("z").unwrap().get_cast::<String>(0).unwrap();
        println!("{}", z);*/

        //println!("{}", x.get::<BencodeObject>(2).unwrap().get::<String>("x").unwrap());

        //SOMEHOW OBTAIN OBJECTS AND ARRAYS
        //FIX Vec<u8> for being bytes only...

        /*
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
        */
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
