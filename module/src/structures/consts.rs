// 定数の宣言: 「const」キーワードを使用する
pub const ROOM_TEMPERATURE: f64 = 20.0; // degrees Celsius

// staticもエクスポートできるが、通常の定数と共存できない
// プログラムが実行を開始する前に用意され、終了するまで生き残る変数だ
// pub static ROOM_TEMPERATURE: f64 = 68.0; // degre、es Fahrenheit
