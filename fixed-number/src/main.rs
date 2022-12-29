fn main() {
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

    // thread 'main' panicked at 'attempt to multiply with overflow'
    // デバックビルドの場合は整数演算中のオーバーフローを検知してパニックを発生させる
    // let mut i = 1;
    // loop {
    //     i *= 10;
    // }

    let mut i: i32 = 1;
    loop {
        // panic: multiplication overflowed (in any build)  パニック：乗算でパニック（いずれのビルドでも）
        // パニック時に挙動を指定することができる
        // Option<Self>を返している
        i = i.checked_mul(10).expect("multiplication overflowed");
    }

    println!("{}", (2.0_f64).sqrt()); // 1.4142135623730951
    println!("{}", f64::sqrt(2.0)); // 1.4142135623730951
}

#[test]
fn test_sample() {
    // チェック付き演算
    // 10と20の和はu8で表現できる
    assert_eq!(10_u8.checked_add(20), Some(30));

    // 100と200の和は表現できない
    assert_eq!(100_u8.checked_add(200_u8), None);

    // ラップ演算
    // 数学的に正しい答えを値の範囲で助産した余を返す
    // 1つ目の積はu16で表現できる
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    // 2つ目の積はu16には収まらないので、250000 modulo 2¹⁶が得られる
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // 符号付き型に対する演算では、ラップした結果符号が変わることがある
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    // ビット単位のシフト演算では、シフトする量が値の範囲に収まるようにラップされる
    // 16ビットの型に対して17ビットシフトしようとすると、1ビットシフトされる
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // 飽和演算
    // 数学的に正しい結果に対して、その型で表現できる最も近い値を返す
    // 結果はその型の最大値もしくは最小値で「クランプ」される
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // オーバーフロー演算
    // タプル（result, overflowed）という結果を返す
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // 浮動小数点型
    // 31515.926e-4f64
    //   31515 => 整数部
    //   .926  => 少数部
    //   e-4   => 指数部
    //   f64   => 型指定子
    // 型を指定していない場合はコンテキストから推論する
    // どちらでも可能な場合は f64 に推論する
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
}
