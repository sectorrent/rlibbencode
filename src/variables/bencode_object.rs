use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io;
use crate::variables::bencode_array::BencodeArray;
use crate::variables::bencode_bytes::BencodeBytes;
use crate::variables::bencode_number::BencodeNumber;
use crate::variables::inter::bencode_variable::{BencodeCast, BencodeVariable};
use crate::variables::inter::bencode_wrapper::{FromBencode, ToBencode};

impl<K: ToBencode, V: ToBencode> ToBencode for HashMap<K, V> {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'm'];
        for (k, v) in self {
            buf.extend(k.to_bencode());
            buf.extend(v.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl<K, V> FromBencode for HashMap<K, V>
where
    K: FromBencode + Eq + Hash,
    V: FromBencode,
{

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if buf[0] != b'm' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for list"));
        }

        let mut _self = Self::new();

        let mut off = 1;
        while buf[off] != b'e' {
            let (k, length) = K::from_bencode(&buf[off..])?;
            off += length;

            let (v, length) = V::from_bencode(&buf[off..])?;
            off += length;

            _self.insert(k, v);
        }

        Ok((_self, off + 2))
    }
}

impl<K: ToBencode, V: ToBencode> ToBencode for BTreeMap<K, V> {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'm'];
        for (k, v) in self {
            buf.extend(k.to_bencode());
            buf.extend(v.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl<K, V> FromBencode for BTreeMap<K, V>
where
    K: FromBencode + Eq + Hash + Ord,
    V: FromBencode,
{

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if buf[0] != b'm' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for list"));
        }

        let mut _self = Self::new();

        let mut off = 1;
        while buf[off] != b'e' {
            let (k, length) = K::from_bencode(&buf[off..])?;
            off += length;

            let (v, length) = V::from_bencode(&buf[off..])?;
            off += length;

            _self.insert(k, v);
        }

        Ok((_self, off + 2))
    }
}











pub trait PutObject<K, V> {

    fn put(&mut self, key: K, value: V);
}

pub trait GetObject<K, T> {

    fn get<V: BencodeCast<T>>(&self, key: K) -> Option<V>;
}

//#[derive(Clone)]
pub struct BencodeObject {
    value: HashMap<BencodeBytes, Box<dyn BencodeVariable>>
}

impl BencodeObject {

    pub fn new() -> Self {
        Self {
            value: HashMap::new()
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
}

impl ToBencode for BencodeObject {

    fn to_bencode(&self) -> Vec<u8> {
        let mut buf = vec![b'm'];
        for (k, v) in &self.value {
            buf.extend(k.to_bencode());
            buf.extend(v.to_bencode());
        }
        buf.push(b'e');
        buf
    }
}

impl FromBencode for BencodeObject {

    fn from_bencode(buf: &[u8]) -> io::Result<(Self, usize)> {
        if buf[0] != b'm' {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid prefix for object"));
        }

        let mut value = HashMap::new();

        let mut off = 1;
        while buf[off] != b'e' {
            let (k, length) = BencodeBytes::from_bencode(&buf[off..])?;
            off += length;

            let (v, length) = match buf[off] {
                b'l' => {
                    let (obj, length) = BencodeArray::from_bencode(&buf[off..])?;
                    (obj.upcast(), length)
                }
                b'm' => {
                    let (obj, length) = BencodeObject::from_bencode(&buf[off..])?;
                    (obj.upcast(), length)
                }
                b'i' => {
                    let (obj, length) = BencodeNumber::from_bencode(&buf[off..])?;
                    (obj.upcast(), length)
                }
                b @ b'0'..=b'9' => {
                    let (obj, length) = BencodeBytes::from_bencode(&buf[off..])?;
                    (obj.upcast(), length)
                }
                _ => unimplemented!()
            };
            off += length;

            value.insert(k, v);
        }

        Ok((Self {
            value
        }, 0))
    }
}

macro_rules! impl_bencode_put_object {
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

impl_bencode_put_object!(
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


macro_rules! impl_bencode_get_object {
    ($(($key:ty, $value:ty)),*) => {
        $(
            impl GetObject<$key, $value> for BencodeObject {

                fn get<V: BencodeCast<$value>>(&self, key: $key) -> Option<V> {
                    self.value
                        .get(&BencodeBytes::from(key))?
                        .as_any()
                        .downcast_ref::<$value>()?.parse::<V>().ok()
                }
            }
        )*
    };
}

impl_bencode_get_object!(
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











