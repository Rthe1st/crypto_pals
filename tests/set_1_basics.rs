extern crate crypto_pals;
extern crate base64;
extern crate crypto;

use self::crypto_pals::hex_to_base64::{ encode_to_base64, hex_decode, hex_encode };
use self::crypto_pals::fixed_xor;
use self::crypto_pals::single_byte_xor_cracker;
use self::crypto_pals::display_helpers::{ u8_vec_to_string };
use self::crypto_pals::repeating_xor_cracker;
use self::crypto_pals::aes;
use self::crypto_pals::repeating_xor;
use std::fs::File;
use std::io::{ BufReader, BufRead, Read};
use std::collections::HashMap;

#[test]
fn challenge_1_convert_hex_to_base64() {
    let based_64_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base_64_encoded = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base_64_encoded,encode_to_base64(&hex_decode(based_64_hex)));
}

#[test]
fn challenge_2_fixed_xor() {
    let bytes_1 = &hex_decode("1c0111001f010100061a024b53535009181c");
    let bytes_2 = &hex_decode("686974207468652062756c6c277320657965");
    let xored_hex = "746865206b696420646f6e277420706c6179";
    assert_eq!(xored_hex, hex_encode(&fixed_xor::xor(bytes_1, bytes_2)));
}

#[test]
fn challenge_3_single_byte_xor_cipher() {
    let cipher_text = &hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let solution = single_byte_xor_cracker::crack(cipher_text);
    let plain_text_string = u8_vec_to_string(&solution.plain_text);
    assert_eq!(plain_text_string, "Cooking MC's like a pound of bacon");
    assert_eq!(solution.key, 88);
    assert_eq!(solution.cipher_text, cipher_text.to_vec());
}

#[test]
fn challenge_4_detect_single_character_xor() {
    let mut results: Vec<single_byte_xor_cracker::Solution> = Vec::new();
    let f = File::open("./tests/challenge_4/cipher_texts.txt").unwrap();
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines(){
        let cipher_text = &hex_decode(&line.unwrap());
        let solution = single_byte_xor_cracker::crack(cipher_text);
        results.push(solution);
    }
    results.sort_by(|a, b| a.score.cmp(&(b.score)));
    for best in results.iter().take(1) {
        assert_eq!(u8_vec_to_string(&(best.plain_text)), "Now that the party is jumping\n");
        assert_eq!(u8_vec_to_string(&(best.cipher_text)), "{ZBA]TAA]PETGAL\\F_@XE\\[R?");
    }
}

#[test]
fn challenge_5_repeating_key_xor(){
    let plain_text = String::from("Burning 'em, if you ain't quick and nimble\n\
                      I go crazy when I hear a cymbal");
    let key = String::from("ICE");
    let correct_cipher_text = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                               a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    assert_eq!(repeating_xor::repeating_xor(&plain_text.into_bytes(), key.into_bytes()), hex_decode(correct_cipher_text));
}

#[test]
fn challenge_6_break_repeating_key_xor(){
    let mut file = File::open("./tests/challenge_6/cipher_text.txt").unwrap();
    let mut base64_text = String::new();
    file.read_to_string(&mut base64_text).unwrap();
    base64_text = base64_text.replace("\n", "");
    let cipher_text = base64::decode(&base64_text).unwrap();
    let key = repeating_xor_cracker::crack(&cipher_text, 1).first().unwrap().to_vec();
    let plain_text = repeating_xor::repeating_xor(&cipher_text, key);
    let mut plain_file = File::open("./tests/challenge_6/plain_text.txt").unwrap();
    let mut correct_plain_text = String::new();
    plain_file.read_to_string(&mut correct_plain_text).unwrap();
    assert_eq!(u8_vec_to_string(&plain_text), correct_plain_text);
}

#[test]
fn challenge_7_aes_in_ecb_mode(){
    let mut file = File::open("./tests/challenge_7/cipher_text.txt").unwrap();
    let mut base64_plaintext = String::new();
    file.read_to_string(&mut base64_plaintext).unwrap();
    base64_plaintext = base64_plaintext.replace("\n", "");
    let cipher_text = base64::decode(&base64_plaintext).unwrap();
    let key = b"YELLOW SUBMARINE";

    let plain_text = aes::decrypt_ecb(&cipher_text, key);

    let mut file = File::open("./tests/challenge_7/plain_text.txt").unwrap();
    let mut correct_plain_text = String::new();
    file.read_to_string(&mut correct_plain_text).unwrap();

    assert_eq!(correct_plain_text, String::from_utf8_lossy(&plain_text));
}

#[test]
fn challenge_8_detect_aes_in_ecb_mode(){
    let file = File::open("./tests/challenge_8/cipher_texts.txt").unwrap();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines().map(|l| l.unwrap()) {
        let cipher_text = &hex_decode(&line);
        let mut block_occurences = HashMap::new();
        for block in cipher_text.chunks(16){
            if !block_occurences.contains_key(block){
                block_occurences.insert(block, 1);
            }else{
                let new_count = block_occurences.get(block).unwrap() + 1;
                block_occurences.insert(block, new_count);
            }
        }
        let number_of_blocks = line.len()/16;
        //ciphertext should look random
        //so with only 10 blocks you wouldn't expect any to appear more then once
        if block_occurences.len() < number_of_blocks {
            println!("found ecb candidate");
            println!("{:?}", block_occurences.values());
            println!("{:?}", line);
            return
        }
    }
    assert!(false, "we never found an ecb candidate :(");
}
