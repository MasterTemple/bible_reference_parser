use managers::media_manager::FullBibleOrganizer;

pub use crate::error::{Error, Result};

pub mod books;
pub mod error;
pub mod managers;
pub mod passage;

fn main() {
    // let man = FullBibleOrganizer::<String>::new();
    // man.
    println!("Hello, world!");
}
