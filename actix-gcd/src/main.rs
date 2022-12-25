// 波括弧内に記載した名前を直接使えるようになる
// つまり actix_web::HttpResponseではなく HttpResponse とだけ書いて使える
use actix_web::{web, App, HttpResponse, HttpServer};
// POSTリクエストをデシリアライズするクレートを使えるようにする
use serde::Deserialize;

// serdeクレートはコンパイル時に型を解析して、HTMLフォームがPOSTリクエストに
// 使用する形式のデータからその型のデータ値を取り出すコードを自動生成する
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    // サーバーを実行すると一連のスレッドをスレッドプールとして実行し、
    // ここのスレッドは起動時に新しい App を受け取る
    // クロージャ式は何も指定していないので、ハンドラが登録される App を返す
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        // HTMLではダブルクォート「"」を使うので、生文字列構文を使用する
        // 文字rの後に0個以上のハッシュマーク「#」が続いた後にダブルクォートで始まる文字列
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} \
    is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

/**
 * ユークリッドの互助法
 *
 * 符号なしの64ビット整数の引数に受け取り、同じ型の値を返す。
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // 引数で0を受け取ると、パニックで終了する
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // セミコロンなしの式の場合は、この式が関数の返り値となる
    // Rustでは return は早期リターンの時には使わない
    n
}

// テスト関数であることを示す
// 通常のコンパイル時にはスキップされるが、 cargo test で自動的にコンパイルされ実行される
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}
