use hex_to_base64;

#[test]
fn hex_encode_test(){
    let hex = "49276d206b696c6c";
    let u8_bin = vec![0b01001001,0b00100111,0b01101101,0b00100000,0b01101011,0b01101001,0b01101100,0b01101100];
    assert_eq!(hex, hex_to_base64::hex_encode(&u8_bin));
}

#[test]
fn hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u8_bin = vec![0b01001001,0b00100111,0b01101101,0b00100000,0b01101011,0b01101001,0b01101100,0b01101100];
    assert_eq!(u8_bin, hex_to_base64::hex_decode(hex));
}

#[test]
fn base_64_encode_test() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,hex_to_base64::encode_to_base64(&hex_to_base64::hex_decode(based_64_hex)));
}
