// use nyanbinary_rs::*;
use crate::prelude::*;

#[component]
pub fn swords() -> impl IntoView {
  // Create an array of 8 different swords
  let swords = [
    Sword::Iron,
    Sword::Obsidian,
    Sword::Diamond,
    Sword::Diamond,
    Sword::Diamond,
    Sword::Diamond,
    Sword::Diamond,
  ];

    view! {
      <h1>"Top ten swords"</h1>
      <ol>
        {swords.into_iter().filter(|sword| sword.good_sword()).map(|sword| view! {
          <li>{format!("{sword:?}")}</li>
        }).collect_view()}
      </ol>
    }
}

// Create different swords using enum, [Iron, Obsidian, Diamond, Bronze, Gold]
#[derive(Debug)]
#[derive(PartialEq)]
enum Sword {
  Iron,
  Obsidian,
  Diamond,
  Bronze,
  Gold,
  Silver,
}

// Create a method called good_sword on Sword
impl Sword {
  fn good_sword(&self) -> bool {
    // If sword is obsidian or diamond, we should return true
    // Otherwise, return false
    match self {
      Sword::Obsidian | Sword::Diamond => true,
      _ => false,
    }
  }
}