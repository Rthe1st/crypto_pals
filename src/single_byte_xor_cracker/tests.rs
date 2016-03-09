use hex_to_base64;
use single_byte_xor_cracker;
use std::char;
#[test]
fn xor_crack(){
    let cipher_text = &hex_to_base64::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let solution = single_byte_xor_cracker::crack(cipher_text);
    let plain_text:Vec<char> = solution.plain_text.iter().cloned().map(|binary| {
        char::from_u32(binary as u32).unwrap()
    }).collect();//.cloned().collect::<String>();
    let plain_text_string:String = plain_text.iter().cloned().collect::<String>();
    assert_eq!(plain_text_string, "Cooking MC's like a pound of bacon");
    assert_eq!(solution.key, 88);
}
