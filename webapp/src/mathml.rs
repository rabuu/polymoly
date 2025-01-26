use leptos::either::Either;
use leptos::math::{mi, mn, mo, msup};
use leptos::prelude::*;

use polymoly::{Poly, R};

#[component]
pub fn PolyDisplay(poly: ReadSignal<Poly<R>>) -> impl IntoView {
    let x = poly.get().map_display_parts(
        |c| mn().child(c.to_string()),
        |e| {
            if let Some(e) = e {
                Either::Left(
                    msup()
                        .child(mi().child("x"))
                        .child(mn().child(e.to_string())),
                )
            } else {
                Either::Right(mi().child("x"))
            }
        },
        || mo().child("+"),
    ).collect_view();

    view! { <math> { x } </math> }
}

pub const LETTER_R: &str = "ℝ";
pub const LETTER_Z: &str = "ℤ";

pub fn zmod_string(sub: &str) -> String {
    format!("{LETTER_Z}/{sub}{LETTER_Z}")
}

pub fn ring(s: impl Into<String>, poly: bool) -> impl IntoView {
    view! {
        <math>
            <mi>{s.into()}</mi>
            <Show when=move || poly>
                <mo>"["</mo>
                <mi>"x"</mi>
                <mo>"]"</mo>
            </Show>
        </math>
    }
}
