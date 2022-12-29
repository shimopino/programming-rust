fn main() {
    println!("Hello, world!");
}

#[test]
fn tuple_test() {
    // タプルは定数のインデックスしか使用できない
    // t.i や t[i] のような変数を使用することはできない

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // タプルでは０要素のタプル () も使用される
    // この型は1つしか値を持たないために、ユニット型と呼ばれている
    // 何も値を返さない関数の返り値の型は () となる
    // 省略されることもある
}
