extern crate rand;

use self::rand::distributions::{IndependentSample, Range};
use aes::{encrypt_ecb, encrypt_cbc};
use padding::{ pkcs_7 };

use std::collections::HashMap;

fn random_128_key() -> [u8; 16]{
    let mut key = [0; 16];
    for index in 0..16{
        key[index] = rand::random::<u8>();
    }
    key
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BlockCipherMode {
    Ecb,
    Cbc,
}

pub fn aes_ecb_or_cbc(input: Vec<u8>) -> (Vec<u8>, BlockCipherMode){
    let mut plain_text = input.clone();
    let key = random_128_key();
    let mut rng = rand::thread_rng();
    let prepend_amount = Range::new(5, 11).ind_sample(&mut rng);
    for _ in 0..prepend_amount{
        plain_text.insert(0, rand::random::<u8>());
    }
    let append_amount = Range::new(5, 11).ind_sample(&mut rng);
    for _ in 0..append_amount{
        plain_text.push(rand::random::<u8>());
    }
    plain_text = pkcs_7(plain_text, 16);
    let iv = random_128_key();
    match rand::random() {
        true => (encrypt_ecb(&plain_text, &key), BlockCipherMode::Ecb),
        false => (encrypt_cbc(&plain_text, &key, &iv), BlockCipherMode::Cbc)
    }
}

pub fn detect_ecb_or_cbc(cipher_text: Vec<u8>, block_size: u8) -> BlockCipherMode{
    let mut block_counts = HashMap::new();
    for block in cipher_text.chunks(block_size as usize){
        let count = block_counts.entry(block).or_insert(0);
        *count += 1;
    }
    match block_counts.values().max().unwrap(){
        &1 => BlockCipherMode::Cbc,
        _ => BlockCipherMode::Ecb
    }
}