mod mathml;

use leptos::either::Either;
use leptos::prelude::*;
use polymoly::{Poly, R};

use self::mathml::MathMLPoly;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (lhs, set_lhs) = signal(String::new());
    let (rhs, set_rhs) = signal(String::new());

    let (result, set_result) = signal(Poly::new(polymoly::R, vec![2.0, 3.0, 4.0]));
    let (error, set_error) = signal(None);

    let output = move || {
        if error.get().is_none() {
            Either::Left(view! { <MathMLPoly poly=result /> })
        } else {
            Either::Right(error.get().unwrap())
        }
    };

    let calculate = move |_| {
                let (lhs, rhs) = match parse(&lhs.get(), &rhs.get()) {
                    Ok(ok) => ok,
                    Err(err) => {
                        set_error.set(Some(err));
                        return;
                    }
                };
                set_result.set(lhs + rhs);
                set_error.set(None);
            };


    view! {
        <input type="text"
            prop:placeholder="Left-hand side"
            bind:value=(lhs, set_lhs)
        />
        <input type="text"
            prop:placeholder="Right-hand side"
            bind:value=(rhs, set_rhs)
        />

        <button on:click=calculate >
            "Calculate"
        </button>

        <p>
            { output }
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
