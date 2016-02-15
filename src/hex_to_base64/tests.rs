use hex_to_base64;

#[test]
fn it_works() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let mut hex_bytes:Vec<u8> = vec![0;96/2];
    for (index, character) in hex.chars().enumerate(){
        let modulus = index % 2;
        let shift = 4 * (1 - modulus);
        hex_bytes[index/2] = hex_bytes[index/2] | (character.to_digit(16).unwrap() as u8) << shift;
    }
    let base_64ed = hex_to_base64::encode_to_base64(hex_bytes);
    println!("{}",base_64ed);
    assert_eq!(base_64ed, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
