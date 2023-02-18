#[cfg(test)]
mod tests {
    fn latin1_to_char(latin1: u8) -> char {
        latin1 as char
    }

    #[test]
    fn test_u8_char_conversion() {
        let a: u8 = 65;
        let c = latin1_to_char(a);

        assert_eq!(c, 'A');
    }
}
