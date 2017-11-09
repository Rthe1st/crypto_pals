#[cfg(test)]
mod tests;

use std::error::Error;
use std::collections::HashMap;

fn hamming_distance(bytes_1: &Vec<u8>, bytes_2: &Vec<u8>) -> Result<u32, Box<Error>>{
    if bytes_1.len() != bytes_2.len() {
        return Err(From::from("byte arrays must be equal length for hamming distance to make sense"));
    }
    let mut total_distance = 0;
    for index in 0..bytes_1.len(){
        let different_bits = bytes_1[index] ^ bytes_2[index];
        let mut byte_distance = 0;
        for bit in (0..7).map(|x| (2 as u8).pow(x)){
            if bit & different_bits == bit{
                byte_distance += 1;
            }
        }
        total_distance += byte_distance;
    }
    Ok(total_distance)
}

pub fn crack(cipher_text: &Vec<u8>) -> (Vec<u8>){
    let mut hamming_distances = HashMap::new();
    for key_length in 1..30{
        let mut iterator = cipher_text.chunks(key_length);
        let first_key_length = iterator.next().unwrap();
        let second_key_length = iterator.next().unwrap();
        let hamming_distance = hamming_distance(&(first_key_length.to_vec()), &(second_key_length.to_vec())).unwrap();
        hamming_distances.insert(key_length, hamming_distance);
    }
    let mut results = hamming_distances.iter()
        .collect::<Vec<_>>();
    results.sort_by(|a, &b| a.1.cmp(&b.1).reverse());
    //println!("{:?}", results);
    vec![0,1,2]
}