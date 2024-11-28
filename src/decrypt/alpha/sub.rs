pub fn vigenere(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    for (i, c) in ciphertext.chars().enumerate() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let decrypted_char = if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            ((c as u8).wrapping_sub(base).wrapping_add(26).wrapping_sub((key_char as u8).wrapping_sub(base) % 26) % 26 + base) as char
        } else {
            c
        };
        plaintext.push(decrypted_char);
    }
    plaintext
}
pub fn caesar(ciphertext: &str, shift: u8) -> String {
    ciphertext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            ((c as u8 - base + 26 - shift) % 26 + base) as char
        } else {
            c
        }
    }).collect()
}
pub fn autokey(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    let mut extended_key = key.to_string();
    for (i, c) in ciphertext.chars().enumerate() {
        let key_char = extended_key.chars().nth(i).unwrap();
        let decrypted_char = if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            ((c as u8 - base + 26 - (key_char as u8 - base) % 26) % 26 + base) as char
        } else {
            c
        };
        plaintext.push(decrypted_char);
        extended_key.push(decrypted_char);
    }
    plaintext
}
pub fn atbash(ciphertext: &str) -> String {
    ciphertext.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            (base + 25 - (c as u8 - base)) as char
        } else {
            c
        }
    }).collect()
}