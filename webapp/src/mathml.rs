use leptos::either::Either;
use leptos::math::{mi, mn, mo, msup};
use leptos::prelude::*;

use polymoly::{Poly, R};

#[component]
pub fn MathMLPoly(poly: ReadSignal<Poly<R>>) -> impl IntoView {
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
