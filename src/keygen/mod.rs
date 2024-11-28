use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use std::io::Write;

pub mod variants;
pub mod convert;
pub fn control() -> io::Result<()> {
    let mut line = String::new();
    println!("Enter your Mode \n1. Convert To Rune\n2. Generate New Key Lists\n:");
    let mut b1 = std::io::stdin().read_line(&mut line).unwrap();
   // let mode: i32 = b1.parse().unwrap().expect("Please enter a valid number");
    let mode: i32 = line.trim().parse().expect("Please enter a valid number");
    if mode == 1 {
        /*
        let mut liner = String::new();
        println!("Enter The name of the original keyfile");
        std::io::stdin().read_line(&mut liner).unwrap();
        let keys_file_path = liner.trim();
        let keys_file = File::open(keys_file_path)?;
        */
        let keys_file_path = "lists/keys.txt";
        let keys_file = File::open(keys_file_path)?;
        let mut keys_reader = BufReader::new(keys_file);
        let mut keys_content = String::new();
        keys_reader.read_to_string(&mut keys_content)?;
        let keys: Vec<String> = keys_content.lines().map(|line| line.to_string()).collect();
        /*
        let mut new_file_path = String::new();
        println!("Enter the name of the new keyfile");
        std::io::stdin().read_line(&mut new_file_path).unwrap();
        let new_file_path = new_file_path.trim();
        */
        let new_file_path = "lists/runeKeys.txt";
        convert::keys_to_rune(&keys, &new_file_path);
    }
    Ok(())

}