BencodeRust
========

This is an implementation of Bencode for Rust. Bencode is used for DHTs, Torrents, and Google DataServers. Its a lightweight fast data serialization.
[Wikipedia](https://en.wikipedia.org/wiki/Bencode)

I have also made an implementation of Bencode with [Java](https://github.com/DrBrad/Bencode), [JavaScript](https://github.com/DrBrad/BencodeJS) and [PHP](https://github.com/DrBrad/BencodePHP).

Implemented Types
-----

| Data | Structure | Default Impl |
| ---  | ---       | ---          |
| Vec  | X         | Defines own ordering |
| VecDeque | X     | Defines own ordering |
| LinkedList | X   | Defines own ordering |
| HashMap | X      | Ordering missing but content is ordered by key byte representation. |
| BTreeMap | X     | Defines own ordering |
| Every Number Type | ✔     | i8-64 u8-64 f32-64 usize |
| String Types | ✔     | Premitive & Object & u8 array |

Usage
-----
Here are some examples of how to use the Bencode library.

**Bencode**
```rust
use crate::variables::bencode_variable::Bencode;
use crate::variables::bencode_object::{BencodeObject, PutObject};

fn main() {
    let mut original = BencodeObject::new();
    original.put("a", "foo");
    original.put("b", "bar");
    original.put("c", 456.78);
    let encoded = original.encode();
    println!("{:?}", encoded);

    let decoded = BencodeObject::decode(encoded);
    println!("{}", decoded.to_string());
}
```


