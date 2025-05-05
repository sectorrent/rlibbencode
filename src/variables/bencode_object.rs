use std::any::Any;
use std::{fmt, io};
use std::fmt::Formatter;
use crate::utils::ordered_map::OrderedMap;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_types::BencodeTypes;
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

#[derive(Debug, Clone)]
pub struct BencodeObject {
    value: OrderedMap<BencodeBytes, Box<dyn BencodeVariable>>
}

impl BencodeObject {

    pub fn new() -> Self {
        Self {
            value: OrderedMap::new()
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BencodeBytes, &Box<dyn BencodeVariable>)> {
        self.value.keys().iter().filter_map(move |key| {
            let value = self.value.get(key)?;
            Some((key, value))
        })
    }
}


impl BencodeVariable for BencodeObject {

    fn get_type(&self) -> BencodeTypes {
        BencodeTypes::Object
    }

    fn upcast(self) -> Box<dyn BencodeVariable> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn BencodeVariable> {
        Box::new(self.clone())
    }
}

impl ToBencode for BencodeObject {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'd'];
        for (k, v) in self.value.iter() {
            buf.extend(k.to_bencode());
            buf.extend(v.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl FromBencode for BencodeObject {

    fn from_bencode_with_offset(buf: &[u8]) -> io::Result<(Self, usize)> {
        if buf[0] != b'd' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for object"));
        }

        let mut value = OrderedMap::new();

        let mut off = 1;
        while buf[off] != b'e' {
            let (k, l) = BencodeBytes::from_bencode_with_offset(&buf[off..])?;
            off += l;

            let (v, l) = match buf[off] {
                b'l' => {
                    let (v, l) = BencodeArray::from_bencode_with_offset(&buf[off..])?;
                    (v.upcast(), l)
                },
                b'd' => {
                    let (v, l) = BencodeObject::from_bencode_with_offset(&buf[off..])?;
                    (v.upcast(), l)
                }
                b'i' => {
                    let (v, l) = BencodeNumber::from_bencode_with_offset(&buf[off..])?;
                    (v.upcast(), l)
                }
                _b @ b'0'..=b'9' => {
                    let (v, l) = BencodeBytes::from_bencode_with_offset(&buf[off..])?;
                    (v.upcast(), l)
                }
                _ => unimplemented!()
            };

            off += l;
            value.insert(k, v);
        }

        Ok((Self {
            value
        }, off + 1))
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
    (String, BencodeBytes, &Vec<u8>),
    (String, BencodeBytes, &[u8]),

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
    (&String, BencodeBytes, &Vec<u8>),
    (&String, BencodeBytes, &[u8]),

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
    (&str, BencodeBytes, &Vec<u8>),
    (&str, BencodeBytes, &[u8]),

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
    (&[u8], BencodeBytes, &Vec<u8>),
    (&[u8], BencodeBytes, &[u8]),

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
    (&Vec<u8>, BencodeBytes, &Vec<u8>),
    (&Vec<u8>, BencodeBytes, &[u8]),

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
    (Vec<u8>, BencodeBytes, Vec<u8>),
    (Vec<u8>, BencodeBytes, &Vec<u8>),
    (Vec<u8>, BencodeBytes, &[u8])
);


macro_rules! impl_bencode_bsize_put_object_primitive {
    ($($type:ty)*) => {
        $(
            impl<const N: usize> PutObject<$type, [u8; N]> for BencodeObject {

                fn put(&mut self, key: $type, value: [u8; N]) {
                    self.value.insert(BencodeBytes::from(key), Box::new(BencodeBytes::from(value)));
                }
            }
        )*
    };
}

impl_bencode_bsize_put_object_primitive!(String &String &str &[u8] Vec<u8> &Vec<u8>);

impl<const N: usize> PutObject<BencodeBytes, [u8; N]> for BencodeObject {

    fn put(&mut self, key: BencodeBytes, value: [u8; N]) {
        self.value.insert(key, Box::new(BencodeBytes::from(value)));
    }
}

macro_rules! impl_bencode_bytes_put_object_primitive {
    ($(($key:ty, $_type:ty, $value:ty)),*) => {
        $(
            impl PutObject<$key, $value> for BencodeObject {

                fn put(&mut self, key: $key, value: $value) {
                    self.value.insert(key, Box::new(<$_type>::from(value)));
                }
            }
        )*
    };
}

impl_bencode_bytes_put_object_primitive!(
    (BencodeBytes, BencodeNumber, u8),
    (BencodeBytes, BencodeNumber, u16),
    (BencodeBytes, BencodeNumber, u32),
    (BencodeBytes, BencodeNumber, u64),
    (BencodeBytes, BencodeNumber, u128),
    (BencodeBytes, BencodeNumber, usize),
    (BencodeBytes, BencodeNumber, i8),
    (BencodeBytes, BencodeNumber, i16),
    (BencodeBytes, BencodeNumber, i32),
    (BencodeBytes, BencodeNumber, i64),
    (BencodeBytes, BencodeNumber, i128),
    (BencodeBytes, BencodeNumber, isize),
    (BencodeBytes, BencodeNumber, f32),
    (BencodeBytes, BencodeNumber, f64),

    (BencodeBytes, BencodeBytes, String),
    (BencodeBytes, BencodeBytes, &str),
    (BencodeBytes, BencodeBytes, Vec<u8>),
    (BencodeBytes, BencodeBytes, &Vec<u8>),
    (BencodeBytes, BencodeBytes, &[u8])
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

impl GetObject<&BencodeBytes> for BencodeObject {

    fn get<V: BencodeVariable + 'static>(&self, key: &BencodeBytes) -> Option<&V> {
        self.value
            .get(key)?
            .as_any()
            .downcast_ref::<V>()
    }

    fn get_mut<V: BencodeVariable + 'static>(&mut self, key: &BencodeBytes) -> Option<&mut V> {
        self.value
            .get_mut(key)?
            .as_any_mut()
            .downcast_mut::<V>()
    }
}

impl PutObject<BencodeBytes, Box<dyn BencodeVariable>> for BencodeObject {

    fn put(&mut self, key: BencodeBytes, value: Box<dyn BencodeVariable>) {
        self.value.insert(key, value);
    }
}

impl ObjectOptions<&BencodeBytes> for BencodeObject {

    fn contains_key(&self, key: &BencodeBytes) -> bool {
        self.value.contains_key(key)
    }

    fn remove(&mut self, key: &BencodeBytes) -> Option<Box<dyn BencodeVariable>> {
        self.value.remove(key)
    }
}


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


macro_rules! impl_bencode_bytes_put_object {
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

impl_bencode_bytes_put_object!(
    (BencodeBytes, BencodeArray),
    (BencodeBytes, BencodeObject),
    (BencodeBytes, BencodeNumber),
    (BencodeBytes, BencodeBytes)
);

impl PartialEq for BencodeObject {

    fn eq(&self, other: &Self) -> bool {
        if self.value.len() != other.value.len() {
            return false;
        }

        for ((a_key, a_val), (b_key, b_val)) in self.value.iter().zip(other.value.iter()) {
            if !a_key.eq(b_key) {
                return false;
            }

            if a_val.get_type() != b_val.get_type() {
                return false;
            }

            if !match a_val.get_type() {
                BencodeTypes::Object => a_val.as_any().downcast_ref::<BencodeObject>().unwrap().eq(
                    b_val.as_any().downcast_ref::<BencodeObject>().unwrap()),
                BencodeTypes::Array => a_val.as_any().downcast_ref::<BencodeArray>().unwrap().eq(
                    b_val.as_any().downcast_ref::<BencodeArray>().unwrap()),
                BencodeTypes::Number => a_val.as_any().downcast_ref::<BencodeNumber>().unwrap().eq(
                    b_val.as_any().downcast_ref::<BencodeNumber>().unwrap()),
                BencodeTypes::Bytes => a_val.as_any().downcast_ref::<BencodeBytes>().unwrap().eq(
                    b_val.as_any().downcast_ref::<BencodeBytes>().unwrap()),
            } {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for BencodeObject {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{\r\n")?;

        for (key, val) in self.value.iter() {
            writeln!(f, "{}\r", match val.get_type() {
                BencodeTypes::Number => format!("\t\u{001b}[0;31m{}\u{001b}[0m: \u{001b}[0;33m{}\u{001b}[0m", key, val),
                BencodeTypes::Bytes => format!("\t\u{001b}[0;31m{}\u{001b}[0m: \u{001b}[0;34m{}\u{001b}[0m", key, val),
                BencodeTypes::Array | BencodeTypes::Object => {
                    let val = format!("{}", val).replace("\r\n", "\r\n\t");
                    format!("\t\u{001b}[0;32m{}\u{001b}[0m: {}", key, val)
                }
            })?;
        }

        write!(f, "}}")
    }
}
