use leptos::either::Either;
use leptos::math::{mi, mn, mo, msup};
use leptos::prelude::*;

use polymoly::polynomial::display::DisplayRing;
use polymoly::polynomial::Polynomial;

pub fn render_polynomial<R>(poly: Polynomial<R>) -> impl IntoView
where
    R: DisplayRing,
    R::Element: std::fmt::Display,
{
    let x = poly
        .map_display_parts(
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
        )
        .collect_view();

    view! { <math> { x } </math> }
}

pub const LETTER_R: &str = "ℝ";
pub const LETTER_Z: &str = "ℤ";

pub fn integers_modulo_string(sub: &str) -> String {
    format!("{LETTER_Z}/{sub}{LETTER_Z}")
}

pub fn ring_string(s: impl Into<String>, is_polynomial: bool) -> impl IntoView {
    view! {
        <math>
            <mi>{s.into()}</mi>
            <Show when=move || is_polynomial>
                <mo>"["</mo>
                <mi>"x"</mi>
                <mo>"]"</mo>
            </Show>
        </math>
    }
}
