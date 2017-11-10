use fixed_xor::xor;
use std::collections::HashMap;
use std::char;

//todo: add support for other languages (i.e. arabic)
//todo: function to return n best options

#[derive(Clone)]
pub struct Solution {
    pub plain_text: Vec<u8>,
    pub cipher_text: Vec<u8>,
    pub key: u8,
    pub score: usize,
}

pub fn crack(cipher_text: &[u8]) -> Solution {
    let mut potential_plain_texts = Vec::new();
    for potential_key in 0 .. ((1 << 7) - 1) as u8 {

        let mut potential_plain_text = Vec::new();
        for byte in cipher_text {
            let plain_text = xor(&[*byte], &[potential_key])[0];
            potential_plain_text.push(plain_text);
        }

        let score = relative_score(&potential_plain_text);

        let solution = Solution { plain_text: potential_plain_text, cipher_text: cipher_text.to_vec(), key: potential_key, score: score};
        potential_plain_texts.push(solution);
    }

    potential_plain_texts.iter().min_by_key(|a| a.score).unwrap().clone()
}

//letters are scored on their relative frequency compared to other letter in the text
//i.e. z gets a high score for being least common, even if it appears in 20% of the text (all other letters appear in >20%)
fn relative_score(plain_text: &[u8]) -> usize{

    //from https://www.cl.cam.ac.uk/~mgk25/lee-essays.pdf page 181
    let frequencies = [
        (' ', 0.1217), ('n', 0.0544),
        ('a', 0.0609), ('o', 0.0600),
        ('b', 0.0105), ('p', 0.0195),
        ('c', 0.0284), ('q', 0.0024),
        ('d', 0.0292), ('r', 0.0495),
        ('e', 0.1136), ('s', 0.0568),
        ('f', 0.0179), ('t', 0.0803),
        ('g', 0.0138), ('u', 0.0243),
        ('h', 0.0341), ('v', 0.0097),
        ('i', 0.0544), ('w', 0.0138),
        ('j', 0.0024), ('x', 0.0024),
        ('k', 0.0041), ('y', 0.0130),
        ('l', 0.0292), ('z', 0.0003),
        ('m', 0.0276)
    ];
    let not_in_alphabet_frequency = 0.0657;

    let mut language_percents: HashMap<char, f64> = HashMap::new();
    for &(letter, frequency) in frequencies.iter() {
        language_percents.insert(letter, frequency);
    }

    let mut text_frequencies: HashMap<char, usize> = HashMap::new();
    let mut not_in_alphabet_count = 0;
    for byte in plain_text {
        let byte_as_char = char::from_u32(*byte as u32).unwrap().to_lowercase().next().unwrap();
        if language_percents.contains_key(&byte_as_char) {
            *text_frequencies.entry(byte_as_char).or_insert(0) += 1;
        }else{
            not_in_alphabet_count += 1;
        }
    }

    let not_in_alphabet_score = frequency_score(not_in_alphabet_count, plain_text.len(), not_in_alphabet_frequency);

    text_frequencies.iter().fold(not_in_alphabet_score, |sum, (key, value)| {
        sum + frequency_score(*value, plain_text.len(), *language_percents.get(key).unwrap())
    })

}

fn frequency_score(number_of_times: usize, text_length: usize, language_percent: f64) -> usize{
    let rounding_degree = 10000.0;
    let actual_percent = number_of_times as f64 / text_length as f64;
    let diff = (language_percent - actual_percent).abs();
    (diff * rounding_degree) as usize
}
