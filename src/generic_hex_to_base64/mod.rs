#[cfg(test)]
mod tests;

extern crate num;

use self::num::traits::PrimInt;
use self::num::{ToPrimitive, FromPrimitive};
use std::mem::size_of;
use std::char;

//todo: 17 and 18 from https://codereview.stackexchange.com/questions/120692/matasano-cryptopals-conversion-to-base-64-with-generic-types

pub fn hex_decode<T: PrimInt + FromPrimitive + ToPrimitive>(hex: &str) -> Vec<T> {
    let num_of_nybbles = (size_of::<T>() * 8) / 4;
    let mut hex_bytes = Vec::new();
    let zero = FromPrimitive::from_u64(0).unwrap();
    hex_bytes.resize(hex.len()/num_of_nybbles, zero);
    for (index, character) in hex.chars().enumerate() {
        let modulus = index % num_of_nybbles;
        let shift = 4 * ((num_of_nybbles - 1) - modulus);
        let decoded_hex:T = FromPrimitive::from_u32(character.to_digit(16).unwrap()).unwrap();
        let hex_index = index/num_of_nybbles;
        hex_bytes[hex_index] = hex_bytes[hex_index] | (decoded_hex << shift);
    }
    hex_bytes
}

pub fn make_length_multiple_of_3<T: FromPrimitive>(raw_binary: &mut Vec<T>) {
    while raw_binary.len() % 3 != 0 {
        raw_binary.push(FromPrimitive::from_u64(0).unwrap());
    }
}

fn handle_carried_bits<T: PrimInt + FromPrimitive + ToPrimitive>(carried_over_bits: T, previous_bits_left: usize, binary_element: T) -> (u8, usize) {
    let type_length = size_of::<T>() * 8;
    let bits_left = type_length - (6 - previous_bits_left);
    let bottom_bits =  binary_element >> bits_left;
    let to_encode = carried_over_bits | bottom_bits;
    (ToPrimitive::to_u8(&to_encode).unwrap(), bits_left)
}

//we encode 6 bits at a time, thus we need a multiple of 6 to encode
//as long as T has 2^x bits, and vector has length 3*y:
// (2^x)*3*y = (2^(x-1))*y*6 will always be a multiple of 6
//so check size of T is a power of 2, then ensure number of elements is a multiple of 3
pub fn encode_to_base64<T: PrimInt + FromPrimitive + ToPrimitive>(mut raw_binary: Vec<T>) -> String {
    assert_eq!(size_of::<T>().count_ones(), 1);
    assert!(size_of::<T>() * 8 > 1, "raw binary has elemnts with 1 or less bits, cannot gurantee converting length to multiple of 3");
    make_length_multiple_of_3(&mut raw_binary);
    let mut base_64 = String::new();
    let mut carried_over_bits = FromPrimitive::from_u64(0).unwrap();
    let mut previous_bits_left = 0;
    let six_bit_mask:T = FromPrimitive::from_u64(0b111111).unwrap();
    for element in raw_binary {
        let (to_encode, mut bits_left) = handle_carried_bits(carried_over_bits, previous_bits_left, element);
        base_64.push(encode_to_char(ToPrimitive::to_u8(&to_encode).unwrap()));
        while bits_left >= 6 {
            let to_encode = six_bit_mask & (element >> (bits_left - 6));
            bits_left -= 6;
            base_64.push(encode_to_char(ToPrimitive::to_u8(&to_encode).unwrap()));
        }
        carried_over_bits = six_bit_mask & (element << (6 - bits_left));
        previous_bits_left = bits_left;
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
