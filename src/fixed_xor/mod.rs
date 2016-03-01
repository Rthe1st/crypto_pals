#[cfg(test)]
mod tests;

pub fn xor(bytes_1: &[u8], bytes_2: &[u8]) -> Vec<u8>{
    assert!(bytes_1.len() == bytes_1.len());
    let mut xored_bytes:Vec<u8> = Vec::new();
    for index in 0 .. bytes_1.len() {
        xored_bytes.push(bytes_1[index] ^ bytes_2[index]);
    }
    xored_bytes
}
