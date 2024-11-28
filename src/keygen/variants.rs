use std::fs::File;
use std::io::{self, Write};
use crate::transliterate_to_rune;
// Renamed from generate_key_variants
pub fn generate_keys(keys: &[String]) -> Vec<String> {
    let mut key_variants = Vec::new();
    for key in keys {
        for i in 0..key.len() {
            for j in 0..key.len() {
                if i != j {
                    let mut new_key = key.clone();
                    new_key.replace_range(i..=i, &key[j..=j]);
                    key_variants.push(new_key);
                }
            }
        }
    }
    key_variants
}
// Renamed from generate_swapped_key_variants
pub fn generate_swapped_keys(keys: &[String]) -> Vec<String> {
    let mut key_variants = Vec::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    for key in keys {
        for (i, c) in key.chars().enumerate() {
            for &letter in &alphabet {
                if !key.contains(letter) {
                    let mut new_key = key.clone();
                    new_key.replace_range(i..=i, &format!("{}", letter as usize));
                    key_variants.push(new_key);
                }
            }
        }
    }
    key_variants
}

pub fn save_keys_to_file(keys: &[String], file_path: &str) -> io::Result<()> {
    let key_variants = generate_keys(keys);
    let mut file = File::create(file_path)?;
    for key in key_variants {
        writeln!(file, "{}", key)?;
    }
    Ok(())
}

pub fn save_swapped_keys_to_file(keys: &[String], file_path: &str) -> io::Result<()> {
    let key_variants = generate_swapped_keys(keys);
    let mut file = File::create(file_path)?;
    for key in key_variants {
        writeln!(file, "{}", key)?;
    }
    Ok(())
}
pub fn convert_keys_to_rune(keys: &[String], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for key in keys {
        let rune_key = transliterate_to_rune(key);
        writeln!(file, "{}", rune_key)?;
    }
    Ok(())
}