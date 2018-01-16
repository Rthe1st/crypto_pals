pub fn pkcs_7(mut unpadded: Vec<u8>, block_size: u8) -> Vec<u8>{
    //usize % (u8 as usize) will never overflow u8
    let left_over_bytes = (unpadded.len() % block_size as usize) as u8;
    let mut padding_needed = block_size - left_over_bytes;
    if padding_needed == 0 {
        padding_needed = block_size;
    }
    for _ in 0 .. padding_needed {
        unpadded.push(padding_needed);
    }
    unpadded
}

pub fn remove_pkcs_7(mut padded: Vec<u8>) -> Vec<u8>{
    let padding_size = (*padded.last().unwrap()) as usize;
    let left_over_size = padded.len() - padding_size;
    padded.truncate(left_over_size);
    padded
}