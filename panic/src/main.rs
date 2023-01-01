fn main() {
    // ゼロ除算になり、パニックが発生する
    // thread 'main' panicked at 'attempt to divide by zero', src/main.rs:7:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    pirate_share(100, 0);

    // スタックの巻き戻しが発生し、作成された順番と逆順で値がドロップされる
    // 実行中の関数が綺麗になれば、それを呼び出した関数に移動してドロップしていく
    // メインスレッドの場合にはプロセスが終了する
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
