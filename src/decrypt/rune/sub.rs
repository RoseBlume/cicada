use std::collections::HashMap;
use crate::transliterator;
const RUNE_ALPHABET: &[(char, &str, u8)] = &[
    ('ᚠ', "F", 0), ('ᚢ', "V(U)", 1), ('ᚦ', "TH", 2), ('ᚩ', "O", 3), ('ᚱ', "R", 4),
    ('ᚳ', "C(K)", 5), ('ᚷ', "G", 6), ('ᚹ', "W", 7), ('ᚻ', "H", 8), ('ᚾ', "N", 9),
    ('ᛁ', "I", 10), ('ᛄ', "J", 11), ('ᛇ', "EO", 12), ('ᛈ', "P", 13), ('ᛉ', "X", 14),
    ('ᛋ', "S(Z)", 15), ('ᛏ', "T", 16), ('ᛒ', "B", 17), ('ᛖ', "E", 18), ('ᛗ', "M", 19),
    ('ᛚ', "L", 20), ('ᛝ', "NG(ING)", 21), ('ᛞ', "D", 23), ('ᛟ', "OE", 22), ('ᚪ', "A", 24),
    ('ᚫ', "AE", 25), ('ᚣ', "Y", 26), ('ᛠ', "EA", 28), ('ᛡ', "IA(IO)", 27)
];

fn build_rune_map() -> HashMap<char, u8> {
    let mut map = HashMap::new();
    for &(rune, _, value) in RUNE_ALPHABET {
        map.insert(rune, value);
    }
    map
}

fn build_reverse_rune_map() -> HashMap<u8, char> {
    let mut map = HashMap::new();
    for &(rune, _, value) in RUNE_ALPHABET {
        map.insert(value, rune);
    }
    map
}

pub fn caesar(text: &str, shift: u8) -> String {
    let rune_map = build_rune_map();
    let reverse_rune_map = build_reverse_rune_map();
    let mut decrypted_text = String::new();

    for ch in text.chars() {
        if let Some(&value) = rune_map.get(&ch) {
            let decrypted_value = (value + shift) % 29;
            if let Some(&decrypted_rune) = reverse_rune_map.get(&decrypted_value) {
                decrypted_text.push(decrypted_rune);
            }
        } else {
            decrypted_text.push(ch);
        }
    }

    transliterator::transliterate_to_alpha(&decrypted_text)
}
pub fn atbash(text: &str) -> String {
    let rune_map = build_rune_map();
    let reverse_rune_map = build_reverse_rune_map();
    let mut decrypted_text = String::new();

    for ch in text.chars() {
        if let Some(&value) = rune_map.get(&ch) {
            let decrypted_value = 28 - value;
            if let Some(&decrypted_rune) = reverse_rune_map.get(&decrypted_value) {
                decrypted_text.push(decrypted_rune);
            }
        } else {
            decrypted_text.push(ch);
        }
    }

    transliterator::transliterate_to_alpha(&decrypted_text)
}

pub fn vigenere(ciphertext: &str, key: &str) -> String {
    let rune_map = build_rune_map();
    let reverse_rune_map = build_reverse_rune_map();
    let mut plaintext = String::new();

    for (i, c) in ciphertext.chars().enumerate() {
        if let Some(&cipher_value) = rune_map.get(&c) {
            let key_char = match key.chars().nth(i % key.len()) {
                Some(c) => c,
                None => return String::from(""),
            };
            if let Some(&key_value) = rune_map.get(&key_char) {
                let decrypted_value = (cipher_value + 29 - key_value) % 29;
                if let Some(&decrypted_rune) = reverse_rune_map.get(&decrypted_value) {
                    plaintext.push(decrypted_rune);
                }
            }
        } else {
            plaintext.push(c);
        }
    }

    transliterator::transliterate_to_alpha(&plaintext).to_string()
}

pub fn autokey(ciphertext: &str, key: &str) -> String {
    let rune_map = build_rune_map();
    let reverse_rune_map = build_reverse_rune_map();
    let mut plaintext = String::new();
    let mut extended_key = key.to_string();

    for (i, c) in ciphertext.chars().enumerate() {
        let key_char = extended_key.chars().nth(i).unwrap();
        let decrypted_char = if let (Some(&c_val), Some(&k_val)) = (rune_map.get(&c), rune_map.get(&key_char)) {
            let decrypted_val = (c_val + 29 - k_val) % 29;
            reverse_rune_map[&decrypted_val]
        } else {
            c
        };
        plaintext.push(decrypted_char);
        extended_key.push(decrypted_char);
    }
    transliterator::transliterate_to_alpha(&plaintext).to_string()
}
