use std::collections::HashMap;

fn encode(value: &str) -> String {
    value.chars().map(|c| {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' => c.to_string(),
            _ => format!("%{:02X}", c as u32)
        }
    }).collect()
}

/**
 * TODO: unit test
 */
pub fn form_body(params: &HashMap<&str, &str>) -> String {
    
    let mut body = String::new();

    for (key, value) in params {
        if !body.is_empty() {
            body.push('&');
        }
        body.push_str(&encode(key));
        body.push('=');
        body.push_str(&encode(value));
    }

    body
}

const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/**
 * TODO: unit test
 */
pub fn base64(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut encoded = String::new();
    let mut temp;

    for chunk in bytes.chunks(3) {
        temp = if chunk.len() == 3 {
            (u32::from(chunk[0]) << 16) | (u32::from(chunk[1]) << 8) | u32::from(chunk[2])
        } else if chunk.len() == 2 {
            (u32::from(chunk[0]) << 16) | (u32::from(chunk[1]) << 8)
        } else {
            u32::from(chunk[0]) << 16
        };

        for i in (0..4).rev() {
            if i > chunk.len() {
                encoded.push('=');
            } else {
                encoded.push(BASE64_CHARS.chars().nth(((temp >> i * 6) & 63) as usize).unwrap());
            }
        }
    }

    encoded
}