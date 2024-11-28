use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use std::io::Write;


pub mod alpha;
pub mod rune;
pub mod validate;
use crate::decrypt;
use crate::transliterator::transliterate_to_alpha;
pub fn delegate() -> io::Result<()> {
    let file_path = "lists/cicada-test.txt";
    //let file_path = "lists/key_variants.txt";
    //let file_path = "lists/cicada.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let mut lineser = String::new();
    println!("Enter your Mode \n1. Decrypt As Runes\n2. Decrypt As Alphabet\n:");
    let mut b1 = std::io::stdin().read_line(&mut lineser).unwrap();
   // let mode: i32 = b1.parse().unwrap().expect("Please enter a valid number");
    let mode: i32 = lineser.trim().parse().expect("Please enter a valid number");
    if mode == 1 {

        let keys_file_path = "lists/runeKeys.txt";
        let keys_file = File::open(keys_file_path)?;
        let mut keys_reader = BufReader::new(keys_file);
        let mut keys_content = String::new();
        keys_reader.read_to_string(&mut keys_content)?;
        let keys: Vec<String> = keys_content.lines().map(|line| line.to_string()).collect();
        let sections: Vec<&str> = content.split('&').collect();
        for (i, section) in sections.iter().enumerate() {
            println!("Processing section {}...", i + 1);
            let transliterated_content = section;
            find_key_and_decrypt_rune(&transliterated_content, &keys)     
        }
    }
    else if mode == 2 {

        let keys_file_path = "lists/keys.txt";
        let keys_file = File::open(keys_file_path)?;
        let mut keys_reader = BufReader::new(keys_file);
        let mut keys_content = String::new();
        keys_reader.read_to_string(&mut keys_content)?;
        let keys: Vec<String> = keys_content.lines().map(|line| line.to_string()).collect();
        let sections: Vec<&str> = content.split('&').collect();
        for (i, section) in sections.iter().enumerate() {
            println!("Processing section {}...", i + 1);
            let transliterated_content = transliterate_to_alpha(section);
            find_key_and_decrypt_alpha(&transliterated_content, &keys)     
        }
    }
    Ok(())
}

fn find_key_and_decrypt_alpha(ciphertext: &str, keys: &[String]) {
    if validate::is_valid_decryption(&ciphertext) {
        println!("{}", ciphertext.to_string());
    }
    println!("Finished Transliteration");
    let mut decrypted_text = alpha::sub::atbash(ciphertext);
    if validate::is_valid_decryption(&decrypted_text) {
        println!("{}", decrypted_text);
    }
    println!("Finished Atbash");
    for shift in 1..=26 {
        decrypted_text = alpha::sub::caesar(ciphertext, shift);
        if validate::is_valid_decryption(&decrypted_text) {
            println!("{}", decrypted_text);
        }
    }
    println!("Finished Caesar");
    for key in keys {
        decrypted_text = alpha::sub::vigenere(ciphertext, &key.to_uppercase());
        if validate::is_valid_decryption(&decrypted_text) {
            println!("{}", decrypted_text);
        }
        /*
        decrypted_text = decrypt_autokey(ciphertext, key);
        if validate::is_valid_decryption(&decrypted_text) {
            println!(decrypted_text);
        }
            */
    }
    println!("Finished Vignere");
}

fn find_key_and_decrypt_rune(ciphertext: &str, keys: &[String]) {
    if validate::is_valid_decryption(&ciphertext) {
        println!("{}", ciphertext.to_string());
    }
    println!("Finished Transliteration");
    let mut decrypted_text = rune::sub::atbash(ciphertext);
    if validate::is_valid_decryption(&decrypted_text) {
        println!("{}", decrypted_text);
    }
    println!("Finished Atbash");
    for shift in 1..=26 {
        decrypted_text = rune::sub::caesar(ciphertext, shift);
        if validate::is_valid_decryption(&decrypted_text) {
            println!("{}", decrypted_text);
        }
    }
    println!("Finished Caesar");
    for key in keys {
        decrypted_text = rune::sub::autokey(ciphertext, &key);
        if validate::is_valid_decryption(&decrypted_text) {
            println!("{}", decrypted_text);
        }
    }
    println!("Finished Autokey");
    for key in keys {
        decrypted_text = rune::sub::vigenere(ciphertext, &key);
        if validate::is_valid_decryption(&decrypted_text) {
            println!("{}", decrypted_text);
        }
        /*
        decrypted_text = decrypt_autokey(ciphertext, key);
        if validate::is_valid_decryption(&decrypted_text) {
            println!(decrypted_text);
        }
            */
    }
    println!("Finished Vignere");
}