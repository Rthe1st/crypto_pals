use hex_to_base64;
use single_byte_xor_cracker;
use display_helpers;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[test]
fn xor_crack(){
    let cipher_text = &hex_to_base64::hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let solution = single_byte_xor_cracker::crack(cipher_text);
    let plain_text_string = display_helpers::u8_vec_to_string(&solution.plain_text);
    assert_eq!(plain_text_string, "Cooking MC's like a pound of bacon");
    assert_eq!(solution.key, 88);
    assert_eq!(solution.cipher_text, cipher_text.to_vec());
}

#[test]
fn detect_xor(){
    let mut results: Vec<single_byte_xor_cracker::Solution> = Vec::new();
    let f = File::open("./src/single_byte_xor_cracker/possible_cipher_texts.txt").unwrap();
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines(){
        let cipher_text = &hex_to_base64::hex_decode(&line.unwrap());
        let solution = single_byte_xor_cracker::crack(cipher_text);
        results.push(solution);
    }
    results.sort_by(|a, b| a.score.cmp(&(b.score)));
    for best in results.iter().take(1) {
        assert_eq!(display_helpers::u8_vec_to_string(&(best.plain_text)), "Now that the party is jumping\n");
        assert_eq!(display_helpers::u8_vec_to_string(&(best.cipher_text)), "{ZBA]TAA]PETGAL\\F_@XE\\[R?");
    }
}
