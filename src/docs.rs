use crate::prelude::*;

#[component]
pub fn docs() -> impl IntoView {
    view! {
  <p>Greetings, one and all!</p>
  <dialog>
  <form method="dialog">
    <button>OK</button>
  </form>
</dialog>
    }
}