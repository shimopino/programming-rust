use std::sync::Mutex;

pub fn locks_in_mutex() {
    // Mutexでベクタを保護する
    let nums = Mutex::new(vec![]);

    {
        // ロックを取得してベクタを保持する
        let mut guard = nums.lock().unwrap();

        // ベクターに値を追加する
        guard.push(10u8);
        guard.push(20u8);
    } // ここでスコープから外れるときにロックは自動的に解放される

    // 再度ロックを取得することができる
    println!("{:?}", nums.lock().unwrap());
}
