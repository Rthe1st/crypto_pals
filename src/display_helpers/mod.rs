#[cfg(test)]
mod tests;

use std::char;

pub fn u8_vec_to_string(binary: &Vec<u8>) -> String{
    binary.iter()
        .map(|&byte| char::from(byte))
        .collect::<String>()
}