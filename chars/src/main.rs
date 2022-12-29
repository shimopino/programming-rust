fn main() {}

#[test]
fn string_test() {
    // Unicodeの１文字を表現するために、32ビットの値の char を使用する
    // ただし、文字列やテキストのストリームに対しては UTF-8 エンコードを用いる
    // String はUTF-8バイト列として保持し、文字の配列として保持するわけではない

    assert_eq!('*', '\x2A');
    assert_eq!('ಠ', '\u{CA0}');

    // charは0x0000から0xD7FFまで、もしくは0xE000から0x10FFFFまでの、Unicodeコードポイントを保持する
    // サロゲートペアの一方（0xD800から0xDFFF）やUnicodeコードスペースの外（0x10FFFFよりも大きい値）を保持することはない
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);

    // 反対への返還は u8 -> char のみ可能
    // u8以外の整数型は、許可されていないUnicodeのコードポイントを指す可能性がある
    // つまり変換には実行時チェックが必要となってしまう

    // 文字クラスに対する便利なメソッドを有している
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}
