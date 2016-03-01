use fixed_xor;

#[test]
fn xor(){
    let hex_1 = "1c0111001f010100061a024b53535009181c";
    let hex_2 = "686974207468652062756c6c277320657965";
    let xored_hex = "746865206b696420646f6e277420706c6179";
    assert_eq!(xored_hex, fixed_xor::xor(hex_1, hex_2));
}

#[test]
fn xor_empty() {
    assert_eq!("", fixed_xor::xor("", ""));
}
