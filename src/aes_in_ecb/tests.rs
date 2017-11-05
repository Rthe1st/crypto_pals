use std::fs::File;
use std::io::prelude::*;

extern crate crypto;
extern crate base64;

use aes_in_ecb;

#[test]
fn decrypt(){
        
    let mut file = File::open("./src/aes_in_ecb/cipher_text.txt").unwrap();
    let mut base64_plaintext = String::new();
    file.read_to_string(&mut base64_plaintext).unwrap();
    base64_plaintext = base64_plaintext.replace("\r\n", "");
    let cipher_text = base64::decode(&base64_plaintext).unwrap();
    let key = b"YELLOW SUBMARINE";

    let plain_text = aes_in_ecb::decrypt_bytes(&cipher_text, key);

    let mut file = File::open("./src/aes_in_ecb/plain_text.txt").unwrap();
    let mut correct_plain_text = String::new();
    file.read_to_string(&mut correct_plain_text).unwrap();

    assert_eq!(correct_plain_text, String::from_utf8_lossy(&plain_text));
}
