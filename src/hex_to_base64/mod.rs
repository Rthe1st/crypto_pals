#[cfg(test)]
mod tests;

use std::char;

pub fn encode_to_base64(mut raw_binary: Vec<u8>) -> String{
    loop{
        match raw_binary.len() % 3{
            0 => break,
            _ => raw_binary.push(0 as u8),
        }
    }
    let mut base_64 = "".to_string();
    for index in (0 ..raw_binary.len()).filter(|&x| x % 3 == 0){
        let encoded_bytes = encode_24_bits([raw_binary[index + 0],raw_binary[index + 1], raw_binary[index + 2]]);
        base_64.push_str(&encoded_bytes);
    }
    base_64
}

pub fn encode_24_bits(raw_binary: [u8; 3]) -> String {
    assert!(raw_binary.len() == 3);
    let mut base_64 = "".to_string();
    let first_6 = raw_binary[0] >> 2;
    base_64.push(encode_to_char(first_6));
    let second_6 = ((raw_binary[0] & 0b00000011) << 4) | (raw_binary[1] >> 4);
    base_64.push(encode_to_char(second_6));
    let third_6 = ((raw_binary[1] & 0b00001111) << 2) | (raw_binary[2] >> 6);
    base_64.push(encode_to_char(third_6));
    let fourth_6 = raw_binary[2] & 0b00111111;
    base_64.push(encode_to_char(fourth_6));
    base_64
}

fn encode_to_char(bits: u8) -> char {
    match bits {
        0 ... 25 => char::from_u32('A' as u32 + bits as u32).unwrap(),
        26 ... 51 => char::from_u32('a' as u32 + (bits - 26) as u32).unwrap(),
        52 ... 61 => char::from_u32('0' as u32 + (bits - 52) as u32).unwrap(),
        62 => '+',
        63 => '/',
        _ => panic!("{} cannot be encoded to a base 64 character"),
    }
}
