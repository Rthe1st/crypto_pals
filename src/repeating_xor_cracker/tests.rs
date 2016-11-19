use repeating_xor_cracker;

#[test]
fn test_hamming_distance(){
    match repeating_xor_cracker::hamming_distance(vec![1,2,3], vec![1,2]){
        Err(err) => assert!(true),
        Ok(distance) => assert!(false),
    }
    match repeating_xor_cracker::hamming_distance(vec![1], vec![1]){
        Err(err) => assert!(false),
        Ok(distance) => assert_eq!(distance, 0),
    }
    let text_1 = String::from("this is a test");
    let text_2 = String::from("wokka wokka!!!");
    match repeating_xor_cracker::hamming_distance(text_1.into_bytes(), text_2.into_bytes()){
        Err(err) => assert!(false),
        Ok(distance) => assert_eq!(distance, 37),
    }
}