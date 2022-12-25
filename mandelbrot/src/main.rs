use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // 第1引数に、ピクセルバッファは可変参照を借用している
    // pixels がバッファベクタの所有者であることを保ったまま、render関数が書き込めるようにしている
    // render(&mut pixels, bounds, upper_left, lower_right);

    // render関数の代わりに並列処理を行うように変更した
    // ８スレッドを使用して並行処理を行う
    let threads = 8;
    // スレッド数に合わせて個々の描画領域に含まれるピクセル数を計算する
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        // |spawner| {...}
        // これはクロージャ式で引数を1つとる
        // spawnerを使用してクロージャ内で新しいスレッドを生成する
        crossbeam::scope(|spawner| {
            // この関数は全ての生成されたスレッドが終了するのを待ってから終了する
            // ここで pixels の一部がスコープからでた後で、生成されたスレッドがアクセスすることがないことがわかる
            // この関数がリターンしてきたら、画像の計算が終了していることが保証される
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                // moveキーワードは、このクロージャが利用する変数の所有権を取得することを意味する
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }
    // ここまで
    // この時点で全てのスレッドが終了していることが保証されるので、次の画像の書き出しをしても安全である

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

/**
* ここでは Complex という下記の構造体を使用している
* ジェネリクスが使用されており、型が動的に決定されることがわかる
*
  struct Complex<T> {
   /// Real portion of the complex number       複素数の実部
   re: T,

   /// Imaginary portion of the complex number  複素数の虚部
   im: T,
  }
*/
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/**
 * 無限ループではなく、繰り返し上限 limit を指定して c がマンデルブロ集合に含まれるのか判定する
 *
 * マンデルブロ集合に含まれない場合は Some(i) を返す
 *
 * Option型は列挙型であり、値が存在している場合は Some(v) として返却する
 */
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        // zの原点からの距離の2乗を返す
        // 半径2の円から出たかどうかを判定するために、距離の2乗の4と比較する
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// 文字列`s`は座標系のペア。`"400x600"`、`"1.0,0.5"`など
///
/// `s`は<left><sep><right>の形でなければならない
/// <sep>は`separator`引数で与えられる文字
/// <left>と<right>は双方とも`T::from_str`でパースできる文字列`separator`はASCII文字でなければならない
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// カンマで分けられた浮動小数点数のペアをパースして複素数を返す
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 出力される画像のピクセルの位置を取り、対応する複素平面上の点を返す。
///
/// `bounds`は、出力画像の幅と高さをピクセル単位
/// `pixel`は画像上の特定のピクセルを(行, 列)ペアの形で指定
/// 仮引数`upper_left`、`lower_right`は、出力画像に描画
///
/// 複素平面を左上と右下で指定
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // Why subtraction here? pixel.1 increases as we go down,
                                                                       // but the imaginary component increases as we go up.†10
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// 矩形範囲のマンデルブロ集合をピクセルのバッファに描画する
///
/// 仮引数`bounds`はバッファ`pixels`の幅と高さを指定
/// バッファ`pixels`はピクセルのグレースケールの値をバイトで保持
/// `upper_left`と`lower_right`はピクセルバッファの左上と右下に対応する複素平面上の点を指定
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// 大きさが`bounds`で指定されたバッファ`pixels`を`filename`で指定されたファイルに書き出す
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    // 処理が失敗した場合は、 write_image 関数全体からエラーを返り値としてリターンする
    // 成功した時に変数 output に値が格納される
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
