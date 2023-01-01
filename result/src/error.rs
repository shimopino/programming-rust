use std::fmt;

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// エラーを表示できるようにする必要あり
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// エラーは該当トレイとの実装が必要
impl std::error::Error for JsonError {}

fn create_error() {
    JsonError {
        message: "expected ']' at end of array".to_string(),
        line: current_line,
        column: current_column,
    }
}

// クレートを使用すれば上記の実装をやってくれる
// use thiserror::Error;
// #[derive(Error, Debug)]
// #[error("{message:} ({line:}, {column})")]
// pub struct JsonError {
//     message: String,
//     line: usize,
//     column: usize,
// }
