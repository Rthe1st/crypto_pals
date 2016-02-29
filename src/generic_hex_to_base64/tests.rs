use hex_to_base64;

#[test]
fn u8_hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u8_bin = vec![0b01001001,0b00100111,0b01101101,0b00100000,0b01101011,0b01101001,0b01101100,0b01101100];
    assert_eq!(u8_bin, hex_to_base64::hex_decode::<u8>(hex));
}

#[test]
fn u16_hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u16_bin = vec![0b0100100100100111,0b0110110100100000,0b0110101101101001,0b0110110001101100];
    assert_eq!(u16_bin, hex_to_base64::hex_decode::<u16>(hex));
}

#[test]
fn u32_hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u32_bin = vec![0b01001001001001110110110100100000,0b01101011011010010110110001101100];
    assert_eq!(u32_bin, hex_to_base64::hex_decode::<u32>(hex));
}

#[test]
fn u64_hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u64_bin = vec![0b0100100100100111011011010010000001101011011010010110110001101100 as u64];
    assert_eq!(u64_bin, hex_to_base64::hex_decode::<u64>(hex));
}

#[test]
fn u8_base_64_encode_test() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,hex_to_base64::encode_to_base64(hex_to_base64::hex_decode::<u8>(based_64_hex)));
}

#[test]
fn u16_base_64_encode_test() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,hex_to_base64::encode_to_base64(hex_to_base64::hex_decode::<u16>(based_64_hex)));
}

#[test]
fn u32_base_64_encode_test() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,hex_to_base64::encode_to_base64(hex_to_base64::hex_decode::<u32>(based_64_hex)));
}

#[test]
fn u64_base_64_encode_test() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,hex_to_base64::encode_to_base64(hex_to_base64::hex_decode::<u64>(based_64_hex)));
}