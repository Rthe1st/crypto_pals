use std::error::Error;

pub fn pkcs_7(unpadded: Vec<u8>, block_size: u8) -> Result<Vec<u8>, Box<Error>>{
    if (unpadded.len() as u8) > block_size {
        return Err(From::from("pkcs_7 is undefined for block sizes that may needed more then 256 bytes of padding"));
    }
    let mut with_padding = unpadded.clone();
    let padding_needed = block_size - (unpadded.len() as u8);
    for _ in 0 .. padding_needed {
        with_padding.push(padding_needed);
    }
    Ok(with_padding)
}