#[cfg(test)]
mod tests;

use fixed_xor;
use std::collections::HashMap;
use std::char;

#[derive(Clone)]
pub struct Solution {
    plain_text: Vec<u8>,
    key: u8,
    score: usize
}

pub fn crack(cipher_text: &[u8]) -> Solution {
    let mut potential_plain_texts = Vec::new();
    for potential_key in 0 .. ((1 << 7) - 1) as u8 {

        let mut potential_plain_text = Vec::new();
        for byte in cipher_text {
            let plain_text = fixed_xor::xor(&[*byte], &[potential_key])[0];
            potential_plain_text.push(plain_text);
        }

        let score = relative_score(&potential_plain_text);

        let solution = Solution { plain_text: potential_plain_text, key: potential_key, score: score};
        potential_plain_texts.push(solution);
    }

    potential_plain_texts.iter().min_by_key(|a| a.score).unwrap().clone()//.plain_text.clone()
}


//leters are scored on their relative frequency compared to other letter in the text
//i.e. z gets a high score for being least common, even if it appears in 20% of the text (all other letters appear in >20%)
fn relative_score(plain_text: &[u8]) -> usize{

    let frequencies = [('e', 0.127), ('t', 0.091), ('a', 0.082), ('o', 0.075),
    ('i', 0.070), ('n', 0.067), ('s', 0.063), ('h', 0.061), ('r', 0.06),
    ('d', 0.043), ('l', 0.04), ('c', 0.028), ('u', 0.028),('m', 0.024),('w', 0.024),('f', 0.022),('g', 0.02),
    ('y', 0.02),('p', 0.019),('b', 0.015),('v', 0.01),('k', 0.008),('j', 0.002),('x', 0.002),('q', 0.001),('z', 0.001)];

    let mut language_percents: HashMap<char, f64> = HashMap::new();
    for &(letter, frequency) in frequencies.iter() {
        language_percents.insert(letter, frequency);
    }

    let mut text_frequencies: HashMap<char, usize> = HashMap::new();
    let mut score:f64 = 0.0;
    let not_in_alphabet_penalty = 0.1;
    for byte in plain_text {
        let byte_as_char = char::from_u32(*byte as u32).unwrap().to_lowercase().next().unwrap();
        if language_percents.contains_key(&byte_as_char) {
            *text_frequencies.entry(byte_as_char).or_insert(0) += 1;
        }else{
            score += not_in_alphabet_penalty;
        }
    }


    for (key, value) in text_frequencies.iter_mut() {
        let actual_percent = *value as f64 / plain_text.len() as f64;
        let language_percent = language_percents.get(key).unwrap();
        let diff:f64 = (language_percent - actual_percent).abs();
        score += diff;
        //println!("key {} , act {}, lang {} diff: {}", key, actual_percent, language_percent, diff);
    }

    (score * 10000.0) as usize

}
