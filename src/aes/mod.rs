#[cfg(test)]
mod tests;

extern crate crypto;
extern crate base64;

use self::crypto::{ aes, blockmodes};
use self::crypto::buffer::{ RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer, BufferResult };

pub fn decrypt_ecb(cipher_text: &[u8], key: &[u8]) -> Vec<u8>{
        
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::NoPadding);

    let mut plain_text = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(&cipher_text);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        plain_text.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    plain_text
}

pub fn encrypt_ecb(plain_text: &[u8], key: &[u8]) -> Vec<u8>{
    let mut encryptor = aes::ecb_encryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::NoPadding);

    let mut cipher_text = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(&plain_text);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        cipher_text.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    cipher_text
}

pub fn encrypt_cbc(plain_text: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>{
    //todo: through an error if not padded to block multiple
    let mut previous_block = iv.to_vec();
    let mut cipher_text = Vec::new();
    for plain_text_block in plain_text.chunks(128/8){
        let mut xored_block = Vec::new();
        for (plain_text_byte, previous_block_byte) in plain_text_block.iter().zip((&previous_block).iter()){
            xored_block.push(plain_text_byte ^ previous_block_byte);
        }
        let cipher_text_block = encrypt_ecb(&xored_block, key);
        cipher_text.append(& mut cipher_text_block.clone());
        previous_block = cipher_text_block;
    }
    cipher_text
}

pub fn decrypt_cbc(cipher_text: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>{
    //todo: through an error if not padded to block multiple
    let mut previous_block = iv.to_vec();
    let mut plain_text = Vec::new();
    for cipher_text_block in cipher_text.chunks(128/8){
        let xored_block = decrypt_ecb(&cipher_text_block, key);
        let mut plain_text_block = Vec::new();
        for (xored_byte, previous_block_byte) in xored_block.iter().zip((&previous_block).iter()){
            plain_text_block.push(xored_byte ^ previous_block_byte);
        }
        plain_text.append(& mut plain_text_block.clone());
        previous_block = cipher_text_block.to_vec();
    }
    plain_text
}