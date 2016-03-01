#[cfg(test)]
mod tests;

extern crate num;

use std::char;

pub fn hex_encode(raw_binary: &[u8]) -> String {
    let mut encoded = String::new();
    let mask = 0b1111;
    for byte in raw_binary {
        let nybbles = vec![((mask << 4) & byte) >> 4, mask & byte];
        for nybble in nybbles {
            encoded.push(match nybble {
                            0 ... 9 => ('0' as u8) + nybble,
                            10 ... 15 => ('a' as u8) + (nybble - 10),
                            _ => panic!("{} cannot be encoded to a hex character", nybble),
                        } as char);
        }
    }
    encoded
}

pub fn hex_decode(hex: &str) -> Vec<u8> {
    let num_of_nybbles = 2;//8 bits = 2 nybbles = 1 byte
    hex.as_bytes().chunks(num_of_nybbles).map(|chunk|{
        let mut decoded:u8 = 0;
        for &byte in chunk {
            decoded = (decoded << 4) | (byte as char).to_digit(16).unwrap() as u8;
        }
        decoded
    }).collect()
}

fn handle_carried_bits(carried_over_bits: u8, previous_bits_left: u8, binary_element: u8) -> (u8, u8) {
    let type_length = 8;//8 bits
    let bits_left = type_length - (6 - previous_bits_left);
    let bottom_bits =  binary_element >> bits_left;
    let to_encode = carried_over_bits | bottom_bits;
    (to_encode, bits_left)
}

//we encode 6 bits at a time, thus we need a multiple of 6 to encode
//as long as T has 2^x bits, and vector has length 3*y:
// (2^x)*3*y = (2^(x-1))*y*6 will always be a multiple of 6
//so check size of T is a power of 2, then ensure number of elements is a multiple of 3
pub fn encode_to_base64(raw_binary: &[u8]) -> String {
    let mut base_64 = String::new();
    let mut carried_over_bits = 0;
    let mut previous_bits_left = 0;
    let six_bit_mask = 0b111111;
    for &element in raw_binary {
        let (to_encode, mut bits_left) = handle_carried_bits(carried_over_bits, previous_bits_left, element);
        base_64.push(encode_to_char(to_encode));
        while bits_left >= 6 {
            let to_encode = six_bit_mask & (element >> (bits_left - 6));
            bits_left -= 6;
            base_64.push(encode_to_char(to_encode));
        }
        carried_over_bits = six_bit_mask & (element << (6 - bits_left));
        previous_bits_left = bits_left;
    }
    if previous_bits_left == 2 {
        base_64.push(encode_to_char(raw_binary.last().unwrap() << 4));
        base_64.push_str("==");
    }else if previous_bits_left == 4 {
        base_64.push(encode_to_char(raw_binary.last().unwrap() << 2));
        base_64.push_str("=");
    }
    base_64
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
