// To make swords a module, do following:
// mod swords;
// use swords::{something_from_swords};

// lib.rs is automatically an module with name <project>
// mod lib as nyanbinary_rs;

use nyanbinary_rs::prelude::*;

fn main() {
    mount_to_body(|| view! {
        <Poggers />
        // <Swords />
        <Docs />
    })
}