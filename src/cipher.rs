pub fn translate(input_text: String, mode: String, key: isize) -> String {
    let symbols =
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.[]");
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
