pub fn repeating_xor(text: Vec<u8>, key: Vec<u8>) -> Vec<u8>{
    let mut result = Vec::new();
    for (text_index, text_byte) in text.iter().enumerate(){
        let key_byte = key[text_index % key.len()];
        result.push(text_byte ^ key_byte);
    }
    result
}