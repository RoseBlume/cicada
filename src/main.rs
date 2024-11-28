use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use std::io::Write;
mod decrypt;
mod keygen;
mod transliterator;
use transliterator::{transliterate_to_alpha, transliterate_to_rune};




fn combine_key_files(output_file: &str, input_files: &[&str]) -> io::Result<()> {
    let mut unique_keys = std::collections::HashSet::new();

    for input_file in input_files {
        let file = File::open(input_file)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(key) = line {
                unique_keys.insert(key);
            }
        }
    }

    let mut output = File::create(output_file)?;
    for key in unique_keys {
        writeln!(output, "{}", key)?;
    }

    Ok(())
}

fn generate_new_keylist(input_file: &str) -> io::Result<Vec<String>> {
    // Read the file line by line
    let file = File::open(input_file)?;
    let lines = io::BufReader::new(file).lines();

    let mut new_keylist = Vec::new();
    let alphabet: Vec<char> = ('a'..='z').collect();

    // Iterate over each key in the file
    for line in lines {
        if let Ok(key) = line {
            let key = key.trim();
            if key.is_empty() {
                continue;
            }

            // Determine the character to replace (first character in the key)
            let first_char = key.chars().next().unwrap();

            // Generate new keys by replacing the first character with each letter of the alphabet
            for &replacement_char in &alphabet {
                // Replace all instances of the first character in the key
                let new_key: String = key
                    .chars()
                    .map(|c| if c == first_char { replacement_char } else { c })
                    .collect();

                new_keylist.push(new_key);
            }
        }
    }

    Ok(new_keylist)
}







fn main() -> io::Result<()> {
    /*
    let file_path = "lists/cicada-test.txt";
    //let file_path = "lists/key_variants.txt";
    //let file_path = "lists/cicada.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    //let keys_file_path = "lists/combined_keys.txt";
    let keys_file_path = "lists/keys.txt";
    let keys_file = File::open(keys_file_path)?;
    let mut keys_reader = BufReader::new(keys_file);
    let mut keys_content = String::new();
    keys_reader.read_to_string(&mut keys_content)?;
    let keys: Vec<String> = keys_content.lines().map(|line| line.to_string()).collect();
    */
    let mut line = String::new();
    println!("Enter your Mode \n1. Key Generation\n2. Decryption\n:");
    let mut b1 = std::io::stdin().read_line(&mut line).unwrap();
   // let mode: i32 = b1.parse().unwrap().expect("Please enter a valid number");
    let mode: i32 = line.trim().parse().expect("Please enter a valid number");
    if mode == 1 {
        keygen::control();
    }
    else if mode == 2 {
        decrypt::delegate();
    }
    
    /*
    //save_key_variants_to_file(&keys, "lists/key_variants.txt")?;
   // save_swapped_key_variants_to_file(&keys, "lists/swapped_key_variants.txt")?;

    let input_file = "lists/keys.txt";
    let output_file = "lists/new_keys.txt";

    // Generate the new keylist
    let new_keys = generate_new_keylist(input_file)?;

    // Write the new keys to a file
    let mut output = File::create(output_file)?;
    for new_key in &new_keys {
        writeln!(&mut output, "{}", new_key)?;
    }
*/
    // Combine key files
    
    
    //let output_file = "lists/combined_keys.txt";
    //let input_files = ["lists/keys.txt", "lists/new_keys.txt", "lists/swapped_key_variants.txt", "lists/key_variants.txt"];
    //combine_key_files(output_file, &input_files)?;
    //println!("New keylist written to {}", output_file);
    
    /*
    let sections: Vec<&str> = content.split('&').collect();
    for (i, section) in sections.iter().enumerate() {
        println!("Processing section {}...", i + 1);
        let transliterated_content = transliterate_to_alpha(section);
        find_key_and_decrypt_alpha(&transliterated_content, &keys) 
    }
    */
    Ok(())
}