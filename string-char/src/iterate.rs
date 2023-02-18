#[test]
fn test_iterate_string() {
    // 整形式なUTF-8の列
    let spacey = "man hat tan";
    let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(spaceless, "manhattan");
}
