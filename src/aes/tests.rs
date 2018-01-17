use aes::{encrypt_ecb, decrypt_ecb, encrypt_cbc, decrypt_cbc};

#[test]
fn test_encrypt_decrypt_ebc(){
    let correct_plain_text = b"QWERTYASDFGZXCVB";
    let key = b"YELLOW SUBMARINE";
    let cipher_text = encrypt_ecb(correct_plain_text, key);
    let plain_text = decrypt_ecb(&cipher_text, key);
    assert_eq!(plain_text, correct_plain_text);

}

#[test]
fn test_encrypt_decrypt_cbc(){
    let correct_plain_text = b"QWERTYASDFGZXCVB";
    let key = b"YELLOW SUBMARINE";
    let iv = b"qweqweqweqqwewwe";
    let cipher_text = encrypt_cbc(correct_plain_text, key, iv);
    let plain_text = decrypt_cbc(&cipher_text, key, iv);
    assert_eq!(plain_text, correct_plain_text);

}