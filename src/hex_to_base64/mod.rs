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
    let type_length = 8;
    let mut bits_left = type_length;
    let mut index = 0;
    loop{
        if bits_left >= 6 {
            let to_encode = 0b111111 & (raw_binary[index] >> (bits_left - 6));
            bits_left -= 6;
            base_64.push(encode_to_char(to_encode));
        }else{
            if index == 3 - 1 { return base_64 }
            let mask = (2 as u8).pow(bits_left + 1) - 1;
            let top_bits = 0b111111 & ((raw_binary[index] & mask) << (6 - bits_left));
            bits_left = type_length - (6 - bits_left);
            let mask = (2 as u8).pow(bits_left + 1) - 1;
            let bottom_bits = mask & (raw_binary[index + 1] >> bits_left);
            index += 1;
            let to_encode = top_bits | bottom_bits;
            base_64.push(encode_to_char(to_encode));
        }
    }
}

fn encode_to_char(bits: u8) -> char {
    match bits {
        0 ... 25 => char::from_u32('A' as u32 + bits as u32).unwrap(),
        26 ... 51 => char::from_u32('a' as u32 + (bits - 26) as u32).unwrap(),
        52 ... 61 => char::from_u32('0' as u32 + (bits - 52) as u32).unwrap(),
        62 => '+',
        63 => '/',
        _ => panic!("{} cannot be encoded to a base 64 character", bits),
    }
}
