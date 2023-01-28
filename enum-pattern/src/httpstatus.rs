use std::{cmp::Ordering, mem::size_of};

// 列挙型はメモリ状では整数として表現される
// 例えば Ordering::Less などは実際には整数として表現されている
// ただし
// 以下のように使用する整数値を指定することも可能である
// 指定しなければ u8 として0から順番に値が指定される
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

// デフォルトでは整数値から列挙型への変換はできない
// ただし整数を絞り込んだ状態であれば、変換することは可能である
fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[test]
fn memory_test() {
    // デフォルトでは u8 に収まるため、1バイトが割り当てられている
    assert_eq!(size_of::<Ordering>(), 1);
    // 404は u8 のサイズに収まらない
    assert_eq!(size_of::<HttpStatus>(), 2);
}

#[test]
fn cast_test() {
    // 整数型へのキャストは可能である
    assert_eq!(HttpStatus::Ok as i32, 200);
}

#[test]
fn reverse_cast_test() {
    assert!(http_status_from_u32(200).is_some());
    assert!(http_status_from_u32(500).is_none());
}
