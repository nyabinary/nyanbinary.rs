use crate::prelude::*;

// A component is just a function with #[component] above it.
#[component]
pub fn poggers() -> impl IntoView {
    let (poggers, set_poggers) = create_signal(0);
    // derived
    // some value is derived from another value
    // let fivple = move || poggers() * 5;

    view! {
      <section class="nes-container with-title">
        <h3 class="title">"Poggers counter"</h3>
        <progress class="nes-progress" value=poggers max="100" />
        <div class="item" id="buttons">
          <button class="nes-btn is-error" on:click=move |_| { set_poggers.update(|n| *n -= 1); }>"Unpoggers"</button>
          <button class="nes-btn is-success" on:click=move |_| { set_poggers.update(|n| *n += 1); }>"Poggers"</button>
        </div>
      </section>
    }
}