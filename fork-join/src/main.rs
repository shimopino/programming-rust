use std::{
    io,
    thread::{self, JoinHandle},
};

fn main() {
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

// fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
//     const NTHREADS: usize = 8;

//     let worklists = aplit_vec_into_chunk(filenames, NTHREADS);
// }
