pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    // 型に対して値をそのまま紐づける型関連定数は以下のように定義する
    // インスタンスに紐づいているわけではない
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };
}

#[test]
fn const_test() {
    let scaled = Vector2::UNIT;

    assert_eq!(scaled.x, 1.0);
    assert_eq!(scaled.y, 1.0);
}
