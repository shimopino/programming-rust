pub struct Ferm {
    // モジュールの外部からアクセスできるフィールドを宣言
    pub roots: String,
    pub stems: u32,

    // プライベートフィールド
    // 構造体が定義されたモジュール内部とそのサブモジュールからはアクセスできる
    name: String,
}

pub fn sample_fn() {
    println!("sample")
}
