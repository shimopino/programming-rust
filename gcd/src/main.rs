fn main() {
    println!("Hello, world!");
}

/**
 * ユークリッドの互助法
 *
 * 符号なしの64ビット整数の引数に受け取り、同じ型の値を返す。
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // 引数で0を受け取ると、パニックで終了する
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // セミコロンなしの式の場合は、この式が関数の返り値となる
    // Rustでは return は早期リターンの時には使わない
    n
}

// テスト関数であることを示す
// 通常のコンパイル時にはスキップされるが、 cargo test で自動的にコンパイルされ実行される
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}
