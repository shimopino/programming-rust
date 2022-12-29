use regex::Regex;

fn main() {
    // 文字列リテラル
}

#[test]
fn literal_test() {
    // ダブルクォートを使用する
    let speech = "\"Ouch!\" said the well.\n";

    // 複数行にまたがってもよい
    // 改行もそのまま出力される
    println!(
        "In the room the women come and go,
        Signing of Mount Abora"
    );

    // 行がバックスラッシュで終わった場合は改行文字と冒頭の空白文字は削除される
    println!(
        "It was a bright, cold day in April, and \
    there were four of us-\
    more or less."
    );

    // 場合によってはWindowsのパスや正規表現などでバックスラッシュをエスケープさせて二重にするのが面倒な時もある
    // その場合には生文字列を使用する
    let default_win_install_path = r"C:\Program Files\Gorillas";
    let patterns = Regex::new(r"\d+(\.\d+)*");

    // ダブルクォート自体を出力したい場合は以下のように # で囲う
    println!(
        r###"
    This raw string started with 'r###"'.
    Therefore it does not end until we reach a quote mark ('"')
    followed immediately by three pound signs ('###'):
"###
    );
}

#[test]
fn byte_test() {
    // bをつけるとバイト文字列となる
    // バイト文字列は、Unicodeテキストからのスライスではなく、u8値の列からのスライスとなる
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}

#[test]
fn memory_test() {
    // 文字列は char の配列としてメモリに格納されるわけではない
    // 文字列は可変調のエンコーディングである UTF-8 で格納される
    // ASCIIは1バイトで格納される
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ";

    // String
    // 可変調なバッファであり、ヒープ領域に確保される
    // noodlesはStringで8バイトのバッファをもっており、うち7バイトを使用する
    // Vec<u8> だと考えることも可能である

    // &str
    // 別のものが所有している連続した UTF-8 テキストへの参照
    // noodlesに属するテキストの最後の6バイトを参照している
    // 他のスライス参照と同様に、実際のデータへのアドレスと、その長さを有するファットポインタ

    // 文字列リテラルは、事前に確保されたテキストを参照する &str
    // プログラムコードと一緒に読み出し専用メモリ領域に置かれる
    // poodlesは文字列リテラルであり、プログラムが実行開始された時に作成され、実行終了まで持続する7バイトの領域を指している

    // lenは確保されるバイト数で計算される
    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    // Stringを返す
    assert_eq!(
        format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string()
    );

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
}
