// 任意の生存期間'eltに対して、生存期間が'eltの参照を保持するExtrema<'elt>を作ることができる
struct Extreme<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extreme<'s>(slice: &'s [i32]) -> Extreme<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[1];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }

    // 返却する構造体も引数sと同じ生存期間を有する
    Extreme { greatest, least }
}

#[test]
fn find_extreme_test() {
    let a = [0, -3, 0, 15, 48];
    // 関数呼び出し時の生存期間はRustが推論する
    let e = find_extreme(&a);

    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}
