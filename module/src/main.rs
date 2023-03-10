use crate::{
    sample::{sample_fn, Ferm},
    structures::stems::stems_fn,
};

mod sample;
mod structures;

fn main() {
    // mod sample
    // これで sample.rs を読み取り、内部の関数や構造体にアクセスすることができる
    sample_fn();

    // missing structure fields: - name
    // let ferm = Ferm {
    //     roots: "sample_ferm".to_string(),
    //     stems: 10,
    // };

    println!("Hello, world!");
    stems_fn();
}

// mod spores {
//     use std::io::Error;

//     /// A cell made by an adult fern. It disperses on the wind as part of
//     /// the fern life cycle. A spore grows into a prothallus -- a whole
//     /// separate organism, up to 5mm across -- which produces the zygote
//     /// that grows into a new fern. (Plant sex is complicated.)†2
//     pub struct Spore {
//         // ...
//     }

//     /// Simulate the production of a spore by meiosis.†3
//     pub fn produce_spore(factory: &mut Sporangium) -> std::io::Error {
//         // ...
//     }

//     /// Extract the genes in a particular spore.†4
//     pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
//         // ...
//     }

//     /// Mix genes to prepare for meiosis (part of interphase).†5
//     fn recombine(parent: &mut Cell) {
//         // ...
//     }
// }

// mod plant_structures {
//     pub mod roots {}
//     pub mod stems {}
//     pub mod leaves {}
// }
