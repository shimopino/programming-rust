#[test]
fn test_push_insert() {
    let mut also_spaceless = "con".to_string();
    // alloc::string::String
    // pub fn push(&mut self, ch: char)
    also_spaceless.push('t');
    assert_eq!(also_spaceless, "cont");

    // alloc::string::String
    // pub fn push_str(&mut self, string: &str)
    also_spaceless.push_str("ri");
    assert_eq!(also_spaceless, "contri");

    // alloc::string::String
    // fn extend<I>(&mut self, iter: I)
    // where
    //     I: IntoIterator<Item = &'a str>,
    also_spaceless.extend("but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    // alloc::string::String
    // pub fn insert(&mut self, idx: usize, ch: char)
    also_spaceless.insert(0, 'I');
    assert_eq!(also_spaceless, "Icontribution");

    // alloc::string::String
    // pub fn insert_str(&mut self, idx: usize, string: &str)
    also_spaceless.insert_str(1, " have ");
    assert_eq!(also_spaceless, "I have contribution");
}
