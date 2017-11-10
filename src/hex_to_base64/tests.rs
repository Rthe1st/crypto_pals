use hex_to_base64::{hex_encode, hex_decode};

#[test]
fn hex_encode_test(){
    let hex = "49276d206b696c6c";
    let u8_bin = vec![0b01001001,0b00100111,0b01101101,0b00100000,0b01101011,0b01101001,0b01101100,0b01101100];
    assert_eq!(hex, hex_encode(&u8_bin));
}

#[test]
fn hex_decode_test(){
    let hex = "49276d206b696c6c";
    let u8_bin = vec![0b01001001,0b00100111,0b01101101,0b00100000,0b01101011,0b01101001,0b01101100,0b01101100];
    assert_eq!(u8_bin, hex_decode(hex));
}