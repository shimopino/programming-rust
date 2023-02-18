#[test]
fn test_push_insert() {
    let mut also_spaceless = "con".to_string();
    also_spaceless.push('t');
    assert_eq!(also_spaceless, "cont");

    also_spaceless.push_str("ri");
    assert_eq!(also_spaceless, "contri");

    also_spaceless.extend("but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    also_spaceless.insert(0, 'I');
    assert_eq!(also_spaceless, "Icontribution");

    also_spaceless.insert_str(1, " have ");
    assert_eq!(also_spaceless, "I have contribution");
}
