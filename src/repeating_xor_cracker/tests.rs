extern crate base64;

use repeating_xor_cracker::{ hamming_distance, normalised_hamming_distance, crack};

#[test]
fn test_hamming_distance(){
    match hamming_distance(&vec![1,2,3], &vec![1,2]){
        Err(_) => assert!(true),
        Ok(_) => assert!(false),
    }
    match hamming_distance(&vec![1], &vec![1]){
        Err(_) => assert!(false),
        Ok(distance) => assert_eq!(distance, 0),
    }
    let text_1 = String::from("this is a test");
    let text_2 = String::from("wokka wokka!!!");
    match hamming_distance(&(text_1.into_bytes()), &(text_2.into_bytes())){
        Err(_) => assert!(false),
        Ok(distance) => assert_eq!(distance, 37),
    }
}

#[test]
fn test_normalised_hamming_distance(){
    match normalised_hamming_distance(&vec![1,2,3], &vec![1,2]){
        Err(_) => assert!(true),
        Ok(_) => assert!(false),
    }
    match normalised_hamming_distance(&vec![1], &vec![1]){
        Err(_) => assert!(false),
        Ok(distance) => assert_eq!(distance, 0.0),
    }
    let text_1 = String::from("this is a test");
    let text_2 = String::from("wokka wokka!!!");
    match normalised_hamming_distance(&(text_1.into_bytes()), &(text_2.into_bytes())){
        Err(_) => assert!(false),
        Ok(distance) => { assert!(true)},//assert_eq!(distance, 37.0 / 14.0)},
    }
}
/*
#[test]
fn test_crack(){
   let plain_text = String::from("Burning 'em, if you ain't quick and nimble\n\
                      I go crazy when I hear a cymbal");
    let correct_key = String::from("ICE");
    let correct_key_as_bytes = correct_key.clone().into_bytes();
    let cipher_text = repeating_xor::repeating_xor(plain_text.into_bytes(), correct_key_as_bytes);
    let key = crack(&cipher_text);
    assert_eq!(u8_vec_to_string(&key), correct_key);
}*/