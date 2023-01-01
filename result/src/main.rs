use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    // unwrapを使用してErrならパニックさせ、Okならその値を返すようにできる
    // ? と異なる点は、間違ってエラーが発生した場合にはパニックを起こして
    // 処理を途中で止めてしまう。

    // mainではResultを通常は返さないため、 ? の構文を使用することはできない
    // expectなどでパニックを起こしてプロセスを終了させる選択肢もある

    // そこでResultを返すように変更したり、自分でエラーの時の処理を行なったりする
}

struct LatLng {
    lat: f64,
    lng: f64,
}

struct WeatherReport {
    report: String,
}

// fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// テキストファイルから数値を読み込む
/// 各行に1つの数値を含んでいることが想定されている
// fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        // 関数の本来の返り値は Result<String, std::io::Error>
        let line = line_result?;
        // 関数の本来の返り値は Result<i64, std::num::ParseIntError>
        numbers.push(line.parse()?);

        // 関数の定義自体には io::Error が定義されている
        // そのため std::num::ParseIntError を返すと型エラーが発生する

        // GenericResult を使うこともできるが呼び出し元からすると
        // どのようなエラーが発生するのか理解することが難しくなってしまう
    }
    Ok(numbers)
}
