pub mod swords;
pub mod poggers;
pub mod docs;
// main.rs:
// use nyanbinary_rs::Swords; // WRONG
// use nyanbinary_rs::swords::Swords;

pub mod prelude {
    pub use leptos::*;
    pub use crate::swords::*;
    pub use crate::poggers::*;
    pub use crate::docs::*;
}