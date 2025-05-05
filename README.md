rlibbencode
========

This is an implementation of Bencode for Rust. Bencode is used for DHTs, Torrents, and Google DataServers. Its a lightweight fast data serialization.
[Wikipedia](https://en.wikipedia.org/wiki/Bencode)

I have also made an implementation of Bencode with [Java](https://github.com/sectorrent/jlibbencode).

Usage
-----
Here are some examples of how to use the Bencode library.

**Bencode**
```rust
use crate::variables::bencode_variable::Bencode;
use crate::variables::bencode_object::{BencodeObject, PutObject};

fn main() {
    let d  = b"d1:v5:0.1.0e";
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
    
    println!("{}", x.to_bencode());
}
```


