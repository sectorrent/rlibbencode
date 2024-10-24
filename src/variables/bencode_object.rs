use std::any::Any;
use std::str::FromStr;
use crate::utils::ordered_map::OrderedMap;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::BencodeVariable;
use crate::variables::inter::bencode_type::BencodeType;

//#[derive(Debug, Clone, PartialEq)]
pub struct BencodeObject {
    m: OrderedMap<BencodeBytes, Box<dyn BencodeVariable>>
}

pub trait PutObject<V> {

    fn put(&mut self, key: &str, value: V);
}

impl BencodeObject {

    const TYPE: BencodeType = BencodeType::Object;

    pub fn new() -> Self {
        Self {
            m: OrderedMap::<BencodeBytes, Box<dyn BencodeVariable>>::new()
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.m.contains_key(&BencodeBytes::from(key))
    }

    pub fn remove(&mut self, key: &str) {
        self.m.remove(&BencodeBytes::from(key));
    }

    pub fn get_number<V>(&self, key: &str) -> Result<V, String> where V: FromStr {
        let key = BencodeBytes::from(key);
        match self.m.get(&key) {
            Some(num) => {
                Ok(num.as_any().downcast_ref::<BencodeNumber>().unwrap().parse::<V>()?)
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
        //self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeNumber>().unwrap().parse::<V>()
    }

    pub fn get_array(&self, key: &str) -> Result<&BencodeArray, String> {
        let key = BencodeBytes::from(key);
        match self.m.get(&key) {
            Some(arr) => {
                Ok(arr.as_any().downcast_ref::<BencodeArray>().unwrap())
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
        //Ok(self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeArray>().unwrap())
    }

    pub fn get_array_mut(&mut self, key: &str) -> Result<&mut BencodeArray, String> {
        let key = BencodeBytes::from(key);
        match self.m.get_mut(&key) {
            Some(arr) => {
                Ok(arr.as_any_mut().downcast_mut::<BencodeArray>().unwrap())
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
        //Ok(self.m.get_mut(&key).unwrap().as_any_mut().downcast_mut::<BencodeArray>().unwrap())
    }

    pub fn get_object(&self, key: &str) -> Result<&BencodeObject, String> {
        let key = BencodeBytes::from(key);
        match self.m.get(&key) {
            Some(obj) => {
                Ok(obj.as_any().downcast_ref::<BencodeObject>().unwrap())
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
        //Ok(self.m.get(&key).unwrap().as_any().downcast_ref::<BencodeObject>().unwrap())
    }

    pub fn get_object_mut(&mut self, key: &str) -> Result<&mut BencodeObject, String> {
        let key = BencodeBytes::from(key);
        match self.m.get_mut(&key) {
            Some(obj) => {
                Ok(obj.as_any_mut().downcast_mut::<BencodeObject>().unwrap())
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
        //Ok(self.m.get_mut(&key).unwrap().as_any_mut().downcast_mut::<BencodeObject>().unwrap())
    }

    pub fn get_bytes(&self, key: &str) -> Result<&[u8], String> {
        let key = BencodeBytes::from(key);
        match self.m.get(&key) {
            Some(str) => {
                Ok(str.as_any().downcast_ref::<BencodeBytes>().unwrap().as_bytes())
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
    }

    pub fn get_string(&self, key: &str) -> Result<&str, String> {
        let key = BencodeBytes::from(key);
        match self.m.get(&key) {
            Some(str) => {
                Ok(str.as_any().downcast_ref::<BencodeBytes>().unwrap().as_str()?)
            },
            None => {
                Err("Variable doesn't exist.".to_string())
            }
        }
    }

    pub fn size(&self) -> usize {
        self.m.len()
    }
}

//REF FROM PARENT FOR OBSERVING UPDATES TO SIZE...
/*
impl<'a> From<BencodeObject<'a>> for BencodeObject<'a> {

    fn from(value: BencodeObject<'a>) -> Self {
        Self {
            m: OrderedMap::<BencodeBytes, BencodeVariable>::new(),
            s: 2
        }
    }
}

impl<'a> From<OrderedMap<BencodeBytes, BencodeVariable<'a>>> for BencodeObject<'a> {

    fn from(value: OrderedMap<BencodeBytes, BencodeVariable<'a>>) -> Self {
        //WE NEED TO COUNT THE SIZE...

        Self {
            m: value
        }
    }
}
*/

impl<const N: usize> PutObject<[u8; N]> for BencodeObject {

    fn put(&mut self, key: &str, value: [u8; N]) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<Vec<u8>> for BencodeObject {

    fn put(&mut self, key: &str, value: Vec<u8>) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<&str> for BencodeObject {

    fn put(&mut self, key: &str, value: &str) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<String> for BencodeObject {

    fn put(&mut self, key: &str, value: String) {
        self.m.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
    }
}

impl PutObject<BencodeArray> for BencodeObject {

    fn put(&mut self, key: &str, value: BencodeArray) {
        self.m.insert(BencodeBytes::from(key), Box::new(value));
    }
}

impl PutObject<BencodeObject> for BencodeObject {

    fn put(&mut self, key: &str, value: BencodeObject) {
        self.m.insert(BencodeBytes::from(key), Box::new(value));
    }
}

macro_rules! impl_object_number {
    ($($type:ty)*) => {
        $(
            impl PutObject<$type> for BencodeObject {

                fn put(&mut self, key: &str, value: $type) {
                    self.m.insert(BencodeBytes::from(key), Box::new(BencodeNumber::from(value)));
                }
            }
        )*
    }
}

impl_object_number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 isize f32 f64);

impl BencodeVariable for BencodeObject {

    fn encode(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.byte_size());
        buf.push(Self::TYPE.prefix());

        for (key, value) in self.m.iter() {
            buf.extend_from_slice(&key.encode());
            buf.extend_from_slice(&value.encode());
        }

        buf.push(Self::TYPE.suffix());
        buf
    }

    fn decode_with_offset(buf: &[u8], off: usize) -> Result<Self, String> where Self: Sized {//Self where Self: Sized {
        let type_ = BencodeType::type_by_prefix(buf[off]).map_err(|e| e.to_string())?;
        if type_ != Self::TYPE {
            return Err("Byte array is not a bencode object.".to_string());
        }

        let mut off = off+1;

        let mut res = OrderedMap::<BencodeBytes, Box<dyn BencodeVariable>>::new();//::with_hasher(Default::default());

        while buf[off] != Self::TYPE.suffix() {
            let key = BencodeBytes::decode_with_offset(buf, off)?;
            off += key.byte_size();
            let type_ = BencodeType::type_by_prefix(buf[off])?;

            let value = match type_ {
                BencodeType::Number => {
                    let value = BencodeNumber::decode_with_offset(buf, off)?;
                    off += value.byte_size();
                    Box::new(value) as Box<dyn BencodeVariable>
                },
                BencodeType::Array => {
                    let value = BencodeArray::decode_with_offset(buf, off)?;
                    off += value.byte_size();
                    Box::new(value) as Box<dyn BencodeVariable>
                },
                BencodeType::Object => {
                    let value = BencodeObject::decode_with_offset(buf, off)?;
                    off += value.byte_size();
                    Box::new(value) as Box<dyn BencodeVariable>
                },
                BencodeType::Bytes => {
                    let value = BencodeBytes::decode_with_offset(buf, off)?;
                    off += value.byte_size();
                    Box::new(value) as Box<dyn BencodeVariable>
                }
            };

            res.insert(key, value);
        }

        Ok(Self {
            m: res
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn byte_size(&self) -> usize {
        let mut s = 2;

        for (key, value) in self.m.iter() {
            s += key.byte_size()+value.byte_size();
        }

        s
    }

    fn to_string(&self) -> String {
        let mut res = "{\r\n".to_string();

        for key in self.m.keys() {
            let value = self.m.get(&key).unwrap();

            if let Some(num) = value.as_any().downcast_ref::<BencodeNumber>() {
                res.push_str(format!("\t\x1b[31m{:?}\x1b[0m: \x1b[33m{}\x1b[0m\r\n", &key.to_string(), num.to_string()).as_str());

            } else if let Some(arr) = value.as_any().downcast_ref::<BencodeArray>() {
                res.push_str(format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), arr.to_string().replace("\r\n", "\r\n\t")).as_str());

            } else if let Some(obj) = value.as_any().downcast_ref::<BencodeObject>() {
                res.push_str(format!("\t\x1b[32m{:?}\x1b[0m: {}\r\n", &key.to_string(), obj.to_string().replace("\r\n", "\r\n\t")).as_str());

            } else if let Some(byt) = value.as_any().downcast_ref::<BencodeBytes>() {
                res.push_str(format!("\t\x1b[31m{:?}\x1b[0m: \x1b[34m{:?}\x1b[0m\r\n", &key.to_string(), byt.to_string()).as_str());
            }
        }

        res.push_str("}");
        res
    }
}
