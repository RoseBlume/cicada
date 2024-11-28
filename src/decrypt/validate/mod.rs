
pub fn is_valid_decryption(text: &str) -> bool {
    let letter_frequencies = calculate_letter_frequencies(text);
    let expected_frequencies = [
        ('A', 8.06), ('B', 1.39), ('C', 3.00), ('D', 3.87), ('E', 12.90), ('F', 1.61),
        ('G', 2.26), ('H', 6.39), ('I', 6.29), ('J', 0.10), ('L', 3.32), ('M', 2.35),
        ('N', 6.68), ('O', 7.93), ('P', 1.32), ('R', 5.97), ('S', 6.19), ('T', 9.90),
        ('V', 4.71), ('W', 3.06), ('X', 0.16), ('Y', 2.55)
    ];

    let mut valid = true;
    let mut differences = 0;
    let variance = 2.0;
    for &(letter, expected_frequency) in &expected_frequencies {
        let actual_frequency = *letter_frequencies.get(&letter).unwrap_or(&0.0);
        if (actual_frequency - expected_frequency).abs() > variance {
            differences += 1;
            if differences > 3 {
                valid = false;
                break;
            }
        }
    }
    valid
}

fn calculate_letter_frequencies(text: &str) -> std::collections::HashMap<char, f64> {
    let mut frequency = std::collections::HashMap::new();
    let total_letters = text.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;

    for c in text.chars().filter(|c| c.is_ascii_alphabetic()) {
        let counter = frequency.entry(c.to_ascii_uppercase()).or_insert(0.0);
        *counter += 1.0;
    }

    for value in frequency.values_mut() {
        *value = (*value / total_letters) * 100.0;
    }

    frequency
}