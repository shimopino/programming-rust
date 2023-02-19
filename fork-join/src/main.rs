use std::{
    sync::Arc,
    thread::{self},
    vec,
};

fn main() {
    process_child_thread();

    let words = vec!["hello".to_string(), "world".to_string()];
    let glossary = vec!["glossary".to_string()];
    // Arc::new によってアトミックな参照カウントを初期化する
    process_child_thread_with_immutable_data(words, Arc::new(glossary));
}

fn process_child_thread() {
    let mut thread_handles = vec![];
    for _ in 0..10 {
        thread_handles.push(thread::spawn(|| {
            println!("hello from a child thread");
        }))
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn process_child_thread_with_immutable_data(words: Vec<String>, glossary: Arc<Vec<String>>) {
    let mut thread_handles = vec![];
    for word in words {
        // 子スレッドごとに参照カウントをインクリメントさせ、生存期間に依存しなくなる
        let glossary_for_child = glossary.clone();
        thread_handles.push(thread::spawn(move || {
            println!(
                "hello from a child thread: {}, {:?}",
                word,
                // 不変の共有データを参照する
                &glossary_for_child
            );
        }))
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
