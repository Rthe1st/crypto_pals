use repeating_xor::repeating_xor;
use hex_to_base64;

#[test]
fn test_xor(){
    let plain_text = String::from("Burning 'em, if you ain't quick and nimble\n\
                      I go crazy when I hear a cymbal");
    let key = String::from("ICE");
    let correct_cipher_text = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                               a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    assert_eq!(repeating_xor(plain_text.into_bytes(), key.into_bytes()), hex_to_base64::hex_decode(correct_cipher_text));
}