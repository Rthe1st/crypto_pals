#[cfg(test)]
mod tests;

use std::error::Error;
use std::collections::HashMap;

use single_byte_xor_cracker;

fn hamming_distance(bytes_1: &Vec<u8>, bytes_2: &Vec<u8>) -> Result<u32, Box<Error>>{
    if bytes_1.len() != bytes_2.len() {
        return Err(From::from("byte arrays must be equal length for hamming distance to make sense"));
    }
    let mut total_distance = 0;
    for index in 0..bytes_1.len(){
        let different_bits = bytes_1[index] ^ bytes_2[index];
        let mut byte_distance = 0;
        for bit in (0..8).map(|x| (2 as u8).pow(x)){
            if bit & different_bits == bit{
                byte_distance += 1;
            }
        }
        total_distance += byte_distance;
    }
    Ok(total_distance)
}

fn normalised_hamming_distance(bytes_1: &Vec<u8>, bytes_2: &Vec<u8>) -> Result<f64, Box<Error>>{
    match hamming_distance(bytes_1, bytes_2){
        Err(error) => return Err(error),
        Ok(distance) => return Ok((distance as f64)/(bytes_1.len() as f64))
    }
}

fn key_size_guesses(cipher_text: &[u8], how_many_guesses: usize, max_key_length: usize) -> Result<Vec<usize>, Box<Error>>{
    if how_many_guesses > max_key_length {
        return Err(From::from("Cannot have more guesses then possible key sizes"));
    }else if max_key_length > cipher_text.len() {
        return Err(From::from("Cannot have a key longer then the cipher text"));
    }
    let mut hamming_distances = HashMap::new();
    for key_length in 1..max_key_length{
        let number_of_averages = cipher_text.len() / key_length;
        let max_index = cipher_text.len() - (cipher_text.len() % key_length);
        let mut iterator = cipher_text[0 .. max_index].chunks(key_length);
        let mut total_distance = 0.0;
        let mut first_chunk = iterator.next().unwrap();
        for chunk in iterator {
            let second_chunk = chunk;
            total_distance += normalised_hamming_distance(&(first_chunk.to_vec()), &(second_chunk.to_vec())).unwrap();
            first_chunk = second_chunk;
        }
        hamming_distances.insert(key_length, total_distance / (number_of_averages as f64));
    }
    let mut results = hamming_distances.iter()
        .collect::<Vec<_>>();
    results.sort_by(|a, &b| a.1.partial_cmp(&b.1).unwrap());
    results.truncate(how_many_guesses);
    let mut key_guesses = vec![];
    for (key, _) in results {
        key_guesses.push(key.clone());
    }
    Ok(key_guesses)
}

pub fn crack(cipher_text: &[u8], key_guesses: usize) -> Vec<Vec<u8>> {
    let mut possible_keys = vec![];
    let possible_key_sizes = key_size_guesses(cipher_text, key_guesses, 30).unwrap();
    for key_size in possible_key_sizes {
        let mut blocks = Vec::new();
        for _ in 0 .. key_size {
            blocks.push(Vec::new());
        }
        for chunk in (&cipher_text).chunks(key_size) {
            for (index, byte) in chunk.iter().enumerate() {
                blocks[index].push(byte.clone());
            }
        }
        let mut key = vec![];
        for block in blocks {
            let result = single_byte_xor_cracker::crack(block.as_slice());
            key.push(result.key);
        }
        possible_keys.push(key);
    }
    possible_keys.clone()
}