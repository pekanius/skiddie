pub fn convert(src: &str) -> String {
    let mut skiddie_text = String::new();
    for c in src.chars() {
        let translated_char = match c {
            'a' | 'A'   => Some("@"),
            'b' | 'B'   => Some("ß"),
            'c' | 'C'   => Some("("),
            'e' | 'E'   => Some("3"),
            'h' | 'H'   => Some("#"),
            'i' | 'I'   => Some("!"),
            'k' | 'K'   => Some("|{"),
            'o' | 'O'   => Some("0"),
            's' | 'S'   => Some("$"),
            'v' | 'V'   => Some("\\/"),
            'x' | 'X'   => Some("}{"),
            'y' | 'Y'   => Some("¥"),
            '!'         => Some("!!!"),
            _   => None,
        };

        if let Some(tc) = translated_char {
            skiddie_text.push_str(tc);
        } else {
            skiddie_text.push(c)
        }
    }
    skiddie_text
}


#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        assert_eq!(crate::convert("Hello world!"), "#3ll0 w0rld!!!");
    }

    #[test]
    fn alphabet() {
        assert_eq!(crate::convert("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"), "@ß(D3FG#!J|{LMN0PQR$TU\\/W}{¥Z@ß(d3fg#!j|{lmn0pqr$tu\\/w}{¥z")
    }
}
