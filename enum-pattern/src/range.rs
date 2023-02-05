#[cfg(test)]
mod tests {
    fn check_char(c: char) -> &'static str {
        match c {
            '0'..='9' => "number",
            'a'..='z' | 'A'..='Z' => "alphabet",
            ' ' | '\t' | '\n' => "white space",
            _ => "not match",
        }
    }

    #[test]
    fn test_c() {
        assert_eq!(check_char('1'), "number");
        assert_eq!(check_char('b'), "alphabet");
        assert_eq!(check_char(' '), "white space");
        assert_eq!(check_char('='), "not match");
    }
}
