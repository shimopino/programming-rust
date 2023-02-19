use rayon::prelude::*;
use std::sync::Arc;

pub fn process_with_rayon(words: Vec<String>, glossary: Arc<Vec<String>>) {
    words
        .par_iter()
        .map(|word| println!("rayon parallel thread: {}", word))
        .reduce_with(|_, _| println!("join result"));
}
