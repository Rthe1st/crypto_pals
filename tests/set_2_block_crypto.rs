extern crate crypto_pals;

use self::crypto_pals::padding::{ pkcs_7 };
use self::crypto_pals::display_helpers::{ u8_vec_to_string };

#[test]
fn challenge_9_implement_pkcs_7_padding(){
    let plain_text = String::from("YELLOW SUBMARINE");
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04", u8_vec_to_string(&pkcs_7(plain_text.into_bytes(), 20).unwrap()));
}