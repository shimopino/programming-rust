// Copy:       コピーする
// Clone:      クローンする
// Debug:     標準出力する
// PartialEq: 部分一致, 今回は f64 型を保持しており、f64はPartialEqを実装している
//            == や != などを使用できる
#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn auto_test() {
    // トレイトの自動実装をすると、自動的にパブリックAPIになってしまう
    let point = Point { x: 0.0, y: 0.0 };

    // 標準出力用のメソッドが自動的に実装される
    println!("{:?}", point); // Point { x: 0.0, y: 0.0 }

    let point1 = Point { x: 0.0, y: 1.0 };
    let point2 = Point { x: 1.0, y: 0.0 };
    println!("{:?} == {:?}: {}", point1, point2, point1 == point2);

    let point3 = point1.clone();
}
