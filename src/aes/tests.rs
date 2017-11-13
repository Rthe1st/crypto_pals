#[test]
fn test_encrypt_decrypt(){
    let correct_plain_text = b"QWERTYASDFGZXCVB";
    let key = b"YELLOW SUBMARINE";
    let cipher_text = aes_in_ecb::encrypt_ecb(&correct_plain_text, key);
    let plain_text = aes_in_ecb::decrypt_ecb(&cipher_text, key);
    assert_eq!(plain_text, correct_plain_text);

}