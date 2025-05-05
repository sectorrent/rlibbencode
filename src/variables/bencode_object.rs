use std::any::Any;
use std::io;
use crate::utils::ordered_map::OrderedMap;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable, FromBencode, ToBencode};

pub trait PutObject<K, V> {

    fn put(&mut self, key: K, value: V);
}

pub trait ObjectOptions<K> {

    fn contains_key(&self, key: K) -> bool;

    fn remove(&mut self, key: K) -> Option<Box<dyn BencodeVariable>>;
}

pub trait GetObjectCast<K, T> {

    fn get_cast<V: BencodeCast<T>>(&self, key: K) -> Option<V>;
}

pub trait GetObject<K> {

    fn get<V: BencodeVariable + 'static>(&self, key: K) -> Option<&V>;

    fn get_mut<V: BencodeVariable + 'static>(&mut self, key: K) -> Option<&mut V>;
}

//#[derive(Clone)]
pub struct BencodeObject {
    value: OrderedMap<BencodeBytes, Box<dyn BencodeVariable>>
}

impl BencodeObject {

    pub fn new() -> Self {
        Self {
            value: OrderedMap::new()
        }
    }
}


impl BencodeVariable for BencodeObject {

    fn upcast(self) -> Box<dyn BencodeVariable> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ToBencode for BencodeObject {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'm'];
        for (k, v) in self.value.iter() {
            buf.extend(k.to_bencode());
            buf.extend(v.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl FromBencode for BencodeObject {

    fn from_bencode_with_offset(buf: &[u8], offset: &mut usize) -> io::Result<Self> {
        if buf[0] != b'm' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for object"));
        }

        let mut value = OrderedMap::new();

        *offset = 1;
        while buf[*offset] != b'e' {
            let k = BencodeBytes::from_bencode_with_offset(&buf[*offset..], offset)?;

            value.insert(k, match buf[*offset] {
                b'l' => BencodeArray::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                b'm' => BencodeObject::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                b'i' => BencodeNumber::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                _b @ b'0'..=b'9' => BencodeBytes::from_bencode_with_offset(&buf[*offset..], offset)?.upcast(),
                _ => unimplemented!()
            });
        }

        *offset += 1;

        Ok(Self {
            value
        })
    }
}

macro_rules! impl_bencode_put_object_primitive {
    ($(($key:ty, $_type:ty, $value:ty)),*) => {
        $(
            impl PutObject<$key, $value> for BencodeObject {

                fn put(&mut self, key: $key, value: $value) {
                    self.value.insert(BencodeBytes::from(key), Box::new(<$_type>::from(value)));
                }
            }
        )*
    };
}

impl_bencode_put_object_primitive!(
    (String, BencodeNumber, u8),
    (String, BencodeNumber, u16),
    (String, BencodeNumber, u32),
    (String, BencodeNumber, u64),
    (String, BencodeNumber, u128),
    (String, BencodeNumber, usize),
    (String, BencodeNumber, i8),
    (String, BencodeNumber, i16),
    (String, BencodeNumber, i32),
    (String, BencodeNumber, i64),
    (String, BencodeNumber, i128),
    (String, BencodeNumber, isize),
    (String, BencodeNumber, f32),
    (String, BencodeNumber, f64),

    (String, BencodeBytes, String),
    (String, BencodeBytes, &str),
    (String, BencodeBytes, Vec<u8>),

    (&String, BencodeNumber, u8),
    (&String, BencodeNumber, u16),
    (&String, BencodeNumber, u32),
    (&String, BencodeNumber, u64),
    (&String, BencodeNumber, u128),
    (&String, BencodeNumber, usize),
    (&String, BencodeNumber, i8),
    (&String, BencodeNumber, i16),
    (&String, BencodeNumber, i32),
    (&String, BencodeNumber, i64),
    (&String, BencodeNumber, i128),
    (&String, BencodeNumber, isize),
    (&String, BencodeNumber, f32),
    (&String, BencodeNumber, f64),

    (&String, BencodeBytes, String),
    (&String, BencodeBytes, &str),
    (&String, BencodeBytes, Vec<u8>),

    (&str, BencodeNumber, u8),
    (&str, BencodeNumber, u16),
    (&str, BencodeNumber, u32),
    (&str, BencodeNumber, u64),
    (&str, BencodeNumber, u128),
    (&str, BencodeNumber, usize),
    (&str, BencodeNumber, i8),
    (&str, BencodeNumber, i16),
    (&str, BencodeNumber, i32),
    (&str, BencodeNumber, i64),
    (&str, BencodeNumber, i128),
    (&str, BencodeNumber, isize),
    (&str, BencodeNumber, f32),
    (&str, BencodeNumber, f64),

    (&str, BencodeBytes, String),
    (&str, BencodeBytes, &str),
    (&str, BencodeBytes, Vec<u8>),

    (&[u8], BencodeNumber, u8),
    (&[u8], BencodeNumber, u16),
    (&[u8], BencodeNumber, u32),
    (&[u8], BencodeNumber, u64),
    (&[u8], BencodeNumber, u128),
    (&[u8], BencodeNumber, usize),
    (&[u8], BencodeNumber, i8),
    (&[u8], BencodeNumber, i16),
    (&[u8], BencodeNumber, i32),
    (&[u8], BencodeNumber, i64),
    (&[u8], BencodeNumber, i128),
    (&[u8], BencodeNumber, isize),
    (&[u8], BencodeNumber, f32),
    (&[u8], BencodeNumber, f64),

    (&[u8], BencodeBytes, String),
    (&[u8], BencodeBytes, &str),
    (&[u8], BencodeBytes, Vec<u8>),

    (&Vec<u8>, BencodeNumber, u8),
    (&Vec<u8>, BencodeNumber, u16),
    (&Vec<u8>, BencodeNumber, u32),
    (&Vec<u8>, BencodeNumber, u64),
    (&Vec<u8>, BencodeNumber, u128),
    (&Vec<u8>, BencodeNumber, usize),
    (&Vec<u8>, BencodeNumber, i8),
    (&Vec<u8>, BencodeNumber, i16),
    (&Vec<u8>, BencodeNumber, i32),
    (&Vec<u8>, BencodeNumber, i64),
    (&Vec<u8>, BencodeNumber, i128),
    (&Vec<u8>, BencodeNumber, isize),
    (&Vec<u8>, BencodeNumber, f32),
    (&Vec<u8>, BencodeNumber, f64),

    (&Vec<u8>, BencodeBytes, String),
    (&Vec<u8>, BencodeBytes, &str),
    (&Vec<u8>, BencodeBytes, Vec<u8>),

    (Vec<u8>, BencodeNumber, u8),
    (Vec<u8>, BencodeNumber, u16),
    (Vec<u8>, BencodeNumber, u32),
    (Vec<u8>, BencodeNumber, u64),
    (Vec<u8>, BencodeNumber, u128),
    (Vec<u8>, BencodeNumber, usize),
    (Vec<u8>, BencodeNumber, i8),
    (Vec<u8>, BencodeNumber, i16),
    (Vec<u8>, BencodeNumber, i32),
    (Vec<u8>, BencodeNumber, i64),
    (Vec<u8>, BencodeNumber, i128),
    (Vec<u8>, BencodeNumber, isize),
    (Vec<u8>, BencodeNumber, f32),
    (Vec<u8>, BencodeNumber, f64),

    (Vec<u8>, BencodeBytes, String),
    (Vec<u8>, BencodeBytes, &str),
    (Vec<u8>, BencodeBytes, Vec<u8>)
);


macro_rules! impl_bencode_get_object_cast {
    ($(($key:ty, $value:ty)),*) => {
        $(
            impl GetObjectCast<$key, $value> for BencodeObject {

                fn get_cast<V: BencodeCast<$value>>(&self, key: $key) -> Option<V> {
                    self.value
                        .get(&BencodeBytes::from(key))?
                        .as_any()
                        .downcast_ref::<$value>()?.parse::<V>().ok()
                }
            }
        )*
    };
}

impl_bencode_get_object_cast!(
    (String, BencodeNumber),
    (String, BencodeBytes),
    (&String, BencodeNumber),
    (&String, BencodeBytes),
    (&str, BencodeNumber),
    (&str, BencodeBytes),
    (&[u8], BencodeNumber),
    (&[u8], BencodeBytes),
    (Vec<u8>, BencodeNumber),
    (Vec<u8>, BencodeBytes),
    (&Vec<u8>, BencodeNumber),
    (&Vec<u8>, BencodeBytes)
);


macro_rules! impl_bencode_get_object {
    ($($type:ty)*) => {
        $(
            impl GetObject<$type> for BencodeObject {

                fn get<V: BencodeVariable + 'static>(&self, key: $type) -> Option<&V> {
                    self.value
                        .get(&BencodeBytes::from(key))?
                        .as_any()
                        .downcast_ref::<V>()
                }

                fn get_mut<V: BencodeVariable + 'static>(&mut self, key: $type) -> Option<&mut V> {
                    self.value
                        .get_mut(&BencodeBytes::from(key))?
                        .as_any_mut()
                        .downcast_mut::<V>()
                }
            }

            impl PutObject<$type, Box<dyn BencodeVariable>> for BencodeObject {

                fn put(&mut self, key: $type, value: Box<dyn BencodeVariable>) {
                    self.value.insert(BencodeBytes::from(key), value);
                }
            }

            impl ObjectOptions<$type> for BencodeObject {

                fn contains_key(&self, key: $type) -> bool {
                    self.value.contains_key(&BencodeBytes::from(key))
                }

                fn remove(&mut self, key: $type) -> Option<Box<dyn BencodeVariable>> {
                    self.value.remove(&BencodeBytes::from(key))
                }
            }
        )*
    };
}

impl_bencode_get_object!(String &String &str &[u8] Vec<u8> &Vec<u8>);



macro_rules! impl_bencode_put_object {
    ($(($key:ty, $value:ty)),*) => {
        $(
            impl PutObject<$key, $value> for BencodeObject {

                fn put(&mut self, key: $key, value: $value) {
                    self.value.insert(BencodeBytes::from(key), Box::new(value));
                }
            }
        )*
    };
}

impl_bencode_put_object!(
    (String, BencodeArray),
    (String, BencodeObject),
    (String, BencodeNumber),
    (String, BencodeBytes),
    (&String, BencodeArray),
    (&String, BencodeObject),
    (&String, BencodeNumber),
    (&String, BencodeBytes),
    (&str, BencodeArray),
    (&str, BencodeObject),
    (&str, BencodeNumber),
    (&str, BencodeBytes),
    (&[u8], BencodeArray),
    (&[u8], BencodeObject),
    (&[u8], BencodeNumber),
    (&[u8], BencodeBytes),
    (Vec<u8>, BencodeArray),
    (Vec<u8>, BencodeObject),
    (Vec<u8>, BencodeNumber),
    (Vec<u8>, BencodeBytes),
    (&Vec<u8>, BencodeArray),
    (&Vec<u8>, BencodeObject),
    (&Vec<u8>, BencodeNumber),
    (&Vec<u8>, BencodeBytes)
);
