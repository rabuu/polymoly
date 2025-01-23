use leptos::prelude::*;
use polymoly::{Poly, R};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (lhs, set_lhs) = signal(String::new());
    let (rhs, set_rhs) = signal(String::new());

    let (output, set_output) = signal(String::new());

    view! {
        <input type="text"
            prop:placeholder="Left-hand side"
            bind:value=(lhs, set_lhs)
        />
        <input type="text"
            prop:placeholder="Right-hand side"
            bind:value=(rhs, set_rhs)
        />

        <button
            on:click=move |_| {
                let (lhs, rhs) = match parse(&lhs.get(), &rhs.get()) {
                    Ok(ok) => ok,
                    Err(err) => {
                        set_output.set(err);
                        return;
                    }
                };
                set_output.set(format!("{}", lhs + rhs));
            }
        >
            "Calculate"
        </button>

        <p>
            {move || output.get()}
        </p>
    }
}

fn parse(lhs: &str, rhs: &str) -> Result<(Poly<R>, Poly<R>), String> {
    let Some(lhs) = Poly::parse(R, lhs) else {
        return Err(format!("Couldn't parse polynomial {lhs}"));
    };

    let Some(rhs) = Poly::parse(R, rhs) else {
        return Err(format!("Couldn't parse polynomial {rhs}"));
    };

    Ok((lhs, rhs))
}
