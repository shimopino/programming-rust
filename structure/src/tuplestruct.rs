// 他のモジュールからアクセス可能にするには下記のように設定する
// pub struct Bounds(pub usize, pub usize);
struct Bounds(usize, usize);

// 新しい型を作成する時にもタプル型構造体を使用することおある
// Vec<u8>のバッファを持ち回るよりも、以下のような型を使って合うセスできるようにする
// 他のバイトバッファを渡ししまうようなミスを避けることができるようになる
struct Ascii(Vec<u8>);

#[test]
fn tuple_test() {
    let image_bounds = Bounds(1024, 768);

    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}
