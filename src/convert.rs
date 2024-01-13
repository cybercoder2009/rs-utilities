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