// 構造体の名称はキャメルケース、あるいはパスカルケース
pub struct GrayScaleMap {
    // デフォルトではフィールドは全てプライベートとなる
    // 他モジュールからアクセスしたい場合hば pub を付与する
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

pub fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayScaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    // 変数名とフィールド名が同じ場合は、JavaScriptのように省略形で記述できる
    GrayScaleMap { pixels, size }
}

#[test]
fn new_map_test() {
    let width = 1024;
    let height = 576;
    let image = new_map((width, height), vec![0; width * height]);

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}
