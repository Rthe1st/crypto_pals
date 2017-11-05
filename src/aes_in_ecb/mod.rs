#[cfg(test)]
mod tests;

extern crate crypto;
extern crate base64;

use self::crypto::{buffer, aes, blockmodes};
use self::crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fn decrypt_bytes(cipher_text: &[u8], key: &[u8]) -> Vec<u8>{
        
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::NoPadding);

    let mut plain_text = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&cipher_text);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

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

