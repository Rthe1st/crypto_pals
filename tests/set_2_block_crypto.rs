extern crate base64;
extern crate crypto_pals;

use self::crypto_pals::padding::{ pkcs_7, remove_pkcs_7 };
use self::crypto_pals::display_helpers::{ u8_vec_to_string };
use self::crypto_pals::aes::{ decrypt_cbc };
use self::crypto_pals::oracles::{ aes_ecb_or_cbc, detect_ecb_or_cbc };

use std::fs::File;
use std::io::{ Read};

#[test]
fn challenge_9_implement_pkcs_7_padding(){
    let plain_text = String::from("YELLOW SUBMARINE");
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04", u8_vec_to_string(&pkcs_7(plain_text.into_bytes(), 20)));
}

#[test]
fn challenge_10_implement_cbc_mode(){
    let key = b"YELLOW SUBMARINE";
    let iv = [0x00; 16];
    let mut file = File::open("./tests/challenge_10/cipher_text.txt").unwrap();
    let mut base64_ciphertext = String::new();
    file.read_to_string(&mut base64_ciphertext).unwrap();
    base64_ciphertext = base64_ciphertext.replace("\n", "");
    let cipher_text = base64::decode(&base64_ciphertext).unwrap();
    let plain_text = remove_pkcs_7(decrypt_cbc(&cipher_text, key, &iv));

    let mut file = File::open("./tests/challenge_10/plain_text.txt").unwrap();
    let mut correct_plain_text = String::new();
    file.read_to_string(&mut correct_plain_text).unwrap();

    assert_eq!(String::from_utf8_lossy(&plain_text),correct_plain_text);
}

#[test]
fn challenge_11_an_ecb_cbc_detection_oracle(){
    //64 As guarantees 2  16 byte block in the middle of just "A"s
    let input = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    for _ in 0..10{
        let (cipher_text, correct_mode) = aes_ecb_or_cbc(input.to_vec());
        assert_eq!(correct_mode, detect_ecb_or_cbc(cipher_text, 16));
    }
}