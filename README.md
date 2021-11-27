# rotcipher-rs
ROT Cipher implementation in rust, supports rot1 to rot25.

# Supported ROT ciphers
ROT1 up to ROT25, it is also possible to make custom ROT ciphers, and here's how:
```rs
use rotcipher::rot;

fn main() {
    let rot_cipher = rot("Hello World!", 55);
    println!("rot (55): {}", rot_cipher)
}
```

# Example
```rs
use rotcipher::macros::{rot4, rot13}

fn main() {
    let rot4_str = rot4!("Hello, World!");
    let rot13_str = rot13!("Hello, World!");

    println!("rot4: {}", rot4_str);
    println!("rot13: {}", rot13_str);
}
```