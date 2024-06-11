pub fn caesar_translate(input_text: String, mode: String, key: isize) -> String {
    // Purpose:    Does the Caesar cipher logic.
    // Parameters: input_text to translate,
    //             mode (encrypt or decrypt), and
    //             key (in size).
    // User Input: None.
    // Prints:     None.
    // Returns:    Translated text as a std::String.
    // Modifies:   Nothing.
    // Calls:      Pure rust, no imports. Hint: rem_euclid
    // Tests:      ./unit_tests/*
    // Status:     Do this one!
    // asserteq!(caesar_translate("abc".to_string, "encrypt".to_string(), 1), "bcd".to_string())
    // asserteq!(caesar_translate("bcd".to_string, "decrypt".to_string(), 1), "abc".to_string())

    let symbols =
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.");
    let mut translated = String::from("");

    for symbol in input_text.chars() {
        if symbols.contains(symbol) {
            let symbolindex = symbols.find(symbol).unwrap();
            if mode.to_string().trim().eq("encrypt") {
                let translated_index =
                    (symbolindex as isize + key).rem_euclid(symbols.len() as isize);
                let translated_char = symbols.chars().nth(translated_index as usize).unwrap();
                translated = translated + &translated_char.to_string();
            } else if mode.to_string().trim().eq("decrypt") {
                let translated_index =
                    (symbolindex as isize - key).rem_euclid(symbols.len() as isize);
                let translated_char = symbols.chars().nth(translated_index as usize).unwrap();
                translated = translated + &translated_char.to_string();
            } else {
                translated = format!("{}{}", translated, symbol.to_string());
            }
        } else {
            translated = format!("{}{}", translated, symbol.to_string());
        }
    }
    translated
}
