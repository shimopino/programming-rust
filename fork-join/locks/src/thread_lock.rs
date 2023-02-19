use std::{sync::Mutex, thread};

pub fn locks_in_thread_mutex() {
    let nums = Mutex::new(vec![]);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                nums.lock().unwrap().push(10_u8);
            })
            .join()
            .unwrap();
        }
    });

    println!("{:?}", nums.lock().unwrap());
}
