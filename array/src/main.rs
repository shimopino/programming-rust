fn main() {
    // Rust
    // [T; N]  => 型TのN個の配列を示す。配列のサイズはコンパイル時に決定され、要素を追加したり削除したりはできない
    // Vec<T>  => Tのベクタで、動的に確保されるT型の値の列を示す。ヒープ上に確保されるので、要素を追加したり削除したり、任意の大きさに変更できる
    // &[T]    => Tの共有スライス、配列やベクタなどの一部の連続した要素への参照を示す、最初の要素へのポインタと、そこからアクセスできる要素の数を持つ
    // &mut[T] => 値を変更できるが、他のスレッドと共有することはできない

    // ベクタは for ループで回せる
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}

#[test]
fn array_test() {
    let lazy_craterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_craterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // 配列に期待されるメソッドは、スライスのメソッドとなっている
    // メソッド探索時に、配列への参照を暗黙的にスライスに変換できるため、直接スライスのメソッドを配列に対して使用できる
    let mut chaos = [3, 5, 4, 1, 2];
    // これは実際にはスライスに定義されている
    // sortは操作対象を参照として受け取る
    // 呼び出し時に暗黙的に &mut [i32] スライスが作成され、これがメソッドに渡される
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn vector_test() {
    // ヒープ上に確保されるサイズを変更できる型Tの配列
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 210 * 11 * 13);

    // Vec::new を使っても生成できる
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    // イテレータが生成する値からも作成できる
    // collectメソッドは型を指定する必要があることが多い
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // ベクタでもスライスのメソッドは使用できる
    // A palindrome!  回文
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    // Reasonable yet disappointing:  こうなるのは当たり前だが、残念だ
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    // Vec<T>を構成する3つの要素
    //   - 要素を保持するためにヒープ上に確保されるバッファへのポインタ
    //   - バッファに保持できる容量
    //   - 現在保持している要素数（ベクタの長さ）
    // バッファが容量の上限に達しているときに新たに要素を追加しようとすると、より大きなバッファが確保され、現在の要素がそちらにコピーされ、ベクタのポインタと容量が更新されて新しいバッファを指すようになり、古いバッファは解放される。

    // あらかじめ要素の数がわかっているときは Vec::with_capacity を使う
    // これで要素を追加してもバッファの再確保は起きない
    // vec! マクロは実はこれを実施している

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    // capacity is now 4
    println!("capacity is now {}", v.capacity());

    // 任意の場所の要素を追加したり削除したりする場合、操作した要素の後ろの要素を全てシフトする必要があるため、ベクタが長い場合には時間がかかる
    let mut v = vec![10, 20, 30, 40, 50];

    // インデックス3の要素を35にする
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // インデックス1の要素を削除する
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    // popは最後の要素を返す
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    // ベクタは for ループで回せる
}

#[test]
fn slice_test() {
    // 配列やベクタのある領域を指すスライスは [T] のように記載する
    // スライスの参照はファットポインタ
    // スライスの最初の要素を指すポインタと、スライスに含まれる要素の数からなる
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    // 配列でもベクタでも動作する
    // そのため同じ型のデータ列に対する関数の引数としてはスライスが適している
    print(sv);
    print(sa);

    // 配列やベクタのスライスへの参照などは、領域をインデックスで指定する
    print(&v[0..2]);
    print(&a[2..]);
    // print!(&sv[1..3]);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}
