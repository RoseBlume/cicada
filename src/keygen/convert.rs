use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use std::io::Write;

use crate::transliterator;

pub fn keys_to_rune(keys: &[String], file_path: &str) -> io::Result<()> {
    let mut file = File::create(&file_path)?;
    for key in keys {
        let rune_key = transliterator::transliterate_to_rune(&key.to_uppercase());
        println!("Result: {}", rune_key);
        writeln!(file, "{}", rune_key)?;
    }
    Ok(())
}