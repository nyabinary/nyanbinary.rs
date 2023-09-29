use leptos::*;

// A component is just a function with #[component] above it
#[component]
fn Poggers() -> impl IntoView {
    let (poggers, set_poggers) = create_signal(0);

    view! {
      <section class="nes-container with-title">
        <h3 class="title">"Poggers counter: "{poggers}</h3>
        <div class="item" id="buttons">
          <button class="nes-btn is-error" on:click=move |_| { set_poggers.update(|n| *n -= 1); }>"Unpoggers"</button>
          <button class="nes-btn is-success" on:click=move |_| { set_poggers.update(|n| *n += 1); }>"Poggers"</button>
        </div>
      </section>
    }
}   

fn main() {
    mount_to_body(|| view! {
        <Poggers />
    })
}
