pub fn transliterate_to_alpha(content: &str) -> String {
    content.chars().map(|rune| match rune {
        'ᚠ' => "F".to_string(),
        'ᚢ' => "V".to_string(),
        'ᚦ' => "TH".to_string(),
        'ᚩ' => "O".to_string(),
        'ᚱ' => "R".to_string(),
        'ᚳ' => "C".to_string(),
        'ᚷ' => "G".to_string(),
        'ᚹ' => "W".to_string(),
        'ᚻ' => "H".to_string(),
        'ᚾ' => "N".to_string(),
        'ᛁ' => "I".to_string(),
        'ᛄ' => "J".to_string(),
        'ᛇ' => "EO".to_string(),
        'ᛈ' => "P".to_string(),
        'ᛉ' => "X".to_string(),
        'ᛋ' => "S".to_string(),
        'ᛏ' => "T".to_string(),
        'ᛒ' => "B".to_string(),
        'ᛖ' => "E".to_string(),
        'ᛗ' => "M".to_string(),
        'ᛚ' => "L".to_string(),
        'ᛝ' => "NG".to_string(),
        'ᛟ' => "OE".to_string(),
        'ᛞ' => "D".to_string(),
        'ᚪ' => "A".to_string(),
        'ᚫ' => "AE".to_string(),
        'ᚣ' => "Y".to_string(),
        'ᛡ' => "IA".to_string(), // IO
        'ᛠ' => "EA".to_string(),
        '/' => "".to_string(),
        _ => rune.to_string(),
    }).collect()
}

pub fn transliterate_to_rune(content: &str) -> String {
    let mut result = content.to_string();
    let replacements = [
        ("ING", 'ᛝ'),
        ("NG", 'ᛝ'),
        ("AE", 'ᚫ'),
        ("EA", 'ᛠ'),
        ("IA", 'ᛡ'),
        ("OE", 'ᛟ'),
        ("EO", 'ᛇ'),
        ("TH", 'ᚦ'),
        ("V", 'ᚢ'),
        ("U", 'ᚢ'),
        ("Y", 'ᚣ'),
        ("A", 'ᚪ'),
        ("D", 'ᛞ'),
        ("L", 'ᛚ'),
        ("M", 'ᛗ'),
        ("E", 'ᛖ'),
        ("B", 'ᛒ'),
        ("T", 'ᛏ'),
        ("S", 'ᛋ'),
        ("Z", 'ᛋ'),
        ("P", 'ᛈ'),
        ("X", 'ᛉ'),
        ("O", 'ᚩ'),
        ("R", 'ᚱ'),
        ("C", 'ᚳ'),
        ("K", 'ᚳ'),
        ("G", 'ᚷ'),
        ("W", 'ᚹ'),
        ("H", 'ᚻ'),
        ("N", 'ᚾ'),
        ("I", 'ᛁ'),
        ("J", 'ᛄ'),
        ("F", 'ᚠ'),
    ];

    for &(alpha, rune) in &replacements {
        result = result.replace(alpha, &rune.to_string());
    }

    result
}
/*
pub fn transliterate_to_rune(content: &str) -> String {
    content.split_whitespace().map(|rune| match rune {
    "ING" => 'ᛝ'.to_string(),
    'NG' => 'ᛝ'.to_string(),
    'AE' => 'ᚫ'.to_string(),
    'EA' => 'ᛠ'.to_string(),
    'IA' => 'ᛡ'.to_string(),
    'OE' => 'ᛟ'.to_string(),
    'EO' => 'ᛇ'.to_string(),
    'TH' => 'ᚦ'.to_string(),
    'V' => 'ᚢ'.to_string(),
    'U' => 'ᚢ'.to_string(),
    'Y' => 'ᚣ'.to_string(),
    'A' => 'ᚪ'.to_string(),
    'D' => 'ᛞ'.to_string(),
    'L' => 'ᛚ'.to_string(),
    'M' => 'ᛗ'.to_string(),
    'E' => 'ᛖ'.to_string(),
    'B' => 'ᛒ'.to_string(),
    'T' => 'ᛏ'.to_string(),
    'S' => 'ᛋ'.to_string(),
    'P' => 'ᛈ'.to_string(),
    'X' => 'ᛉ'.to_string(),
    'O' => 'ᚩ'.to_string(),
    'R' => 'ᚱ'.to_string(),
    'C' => 'ᚳ'.to_string(),
    'K' => 'ᚳ'.to_string(),
    'G' => 'ᚷ'.to_string(),
    'W' => 'ᚹ'.to_string(),
    'H' => 'ᚻ'.to_string(),
    'N' => 'ᚾ'.to_string(),
    'I' => 'ᛁ'.to_string(),
    'J' => 'ᛄ'.to_string(),
    'F' => 'ᚠ'.to_string(),
    _ => rune.to_string(),
    }).collect()
}
    */