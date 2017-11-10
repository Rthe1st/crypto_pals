extern crate base64;

use repeating_xor_cracker::hamming_distance;

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