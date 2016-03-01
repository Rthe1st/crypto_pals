use fixed_xor;
use hex_to_base64;

#[test]
fn xor(){
    let bytes_1 = &hex_to_base64::hex_decode("1c0111001f010100061a024b53535009181c");
    let bytes_2 = &hex_to_base64::hex_decode("686974207468652062756c6c277320657965");
    let xored_hex = "746865206b696420646f6e277420706c6179";
    assert_eq!(xored_hex, hex_to_base64::hex_encode(&fixed_xor::xor(bytes_1, bytes_2)));
}
