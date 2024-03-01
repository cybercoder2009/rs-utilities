pub fn str_to_u8_32(s: &str) -> [u8; 32] {
    let mut array: [u8; 32] = [0u8; 32];
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate().take(32) {
        array[i] = byte;
    }
    array
}

pub fn str_to_u8_64(s: &str) -> [u8; 64] {
    let mut array: [u8; 64] = [0u8; 64];
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate().take(64) {
        array[i] = byte;
    }
    array
}

pub fn hex_str_to_u8_32(hex_str: &str) -> Result<[u8; 32], std::num::ParseIntError> {
    let trucated: &str = if hex_str.len() > 64 { &hex_str[..64] } else { hex_str };
    let mut array: [u8; 32] = [0u8; 32];
    for (i, byte) in trucated.as_bytes().chunks(2).enumerate() {
        array[i] = u8::from_str_radix(std::str::from_utf8(byte).unwrap(), 16)?;
    }
    Ok(array)
}

pub fn u8_32_to_hex_string(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn hex_str_to_u8_64(hex_str: &str) -> Result<[u8; 64], std::num::ParseIntError> {
    let trucated: &str = if hex_str.len() > 128 { &hex_str[..128] } else { hex_str };
    let mut array: [u8; 64] = [0u8; 64];
    for (i, byte) in trucated.as_bytes().chunks(2).enumerate() {
        array[i] = u8::from_str_radix(std::str::from_utf8(byte).unwrap(), 16)?;
    }
    Ok(array)
}

pub fn u8_64_to_hex_string(bytes: &[u8; 64]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}
