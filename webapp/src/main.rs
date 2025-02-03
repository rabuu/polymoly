mod mathml;
mod operation;

use leptos::html::P;
use leptos::prelude::*;

use polymoly::polynomial::display::DisplayRing;
use polymoly::polynomial::parse::ParsableRing;
use polymoly::polynomial::Polynomial;
use polymoly::ring::{Integers, IntegersModuloN, IntegersModuloP, PolynomialRing, Reals};

use operation::{OperandRingType, Operation};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (op, set_op) = signal(Operation::Add);

    let (lhs, set_lhs) = signal(String::new());
    let (rhs, set_rhs) = signal(String::new());

    let (normal_ring, set_normal_ring) = signal(String::from("reals"));
    let (field_ring, set_field_ring) = signal(String::from("reals"));
    let (euclidean_ring, set_euclidean_ring) = signal(String::from("integers"));

    let (n, set_n) = signal(2_usize);
    let do_select_n = move || match op.get().operand_ring_type() {
        OperandRingType::Normal => normal_ring.get() == "modulo",
        OperandRingType::Field => field_ring.get() == "modulo",
        OperandRingType::Euclidean => euclidean_ring.get() == "modulo",
    };

    view! {
    <div>
        <input type="radio" name="operation" value="add" id="op:add" on:input:target=move |ev| set_op.set(ev.target().value().into()) checked />
        <label for="op:add">Add</label>
        <input type="radio" name="operation" value="sub" id="op:sub" on:input:target=move |ev| set_op.set(ev.target().value().into()) />
        <label for="op:add">Subtract</label>
        <input type="radio" name="operation" value="mul" id="op:mul" on:input:target=move |ev| set_op.set(ev.target().value().into()) />
        <label for="op:mul">Multiply</label>
        <input type="radio" name="operation" value="div" id="op:div" on:input:target=move |ev| set_op.set(ev.target().value().into()) />
        <label for="op:mul">Divide</label>
        <input type="radio" name="operation" value="gcd" id="op:gcd" on:input:target=move |ev| set_op.set(ev.target().value().into()) />
        <label for="op:mul">GCD</label>
    </div>

    <input type="text"
        prop:placeholder="Left-hand side"
        bind:value=(lhs, set_lhs)
    />
    <input type="text"
        prop:placeholder="Right-hand side"
        bind:value=(rhs, set_rhs)
    />

    <Show when=move || op.get().operand_ring_type() == OperandRingType::Normal>
        <select
            on:change:target=move |ev| set_normal_ring.set(ev.target().value())
            prop:value=move || normal_ring.get()
        >
            <option value="reals">{ mathml::ring_string(mathml::LETTER_R, true) }</option>
            <option value="integers">{ mathml::ring_string(mathml::LETTER_Z, true) }</option>
            <option value="modulo">{ mathml::ring_string(mathml::integers_modulo_string("n"), true) }</option>
        </select>
    </Show>

    <Show when=move || op.get().operand_ring_type() == OperandRingType::Field>
        <select
            on:change:target=move |ev| set_field_ring.set(ev.target().value())
            prop:value=move || field_ring.get()
        >
            <option value="reals">{ mathml::ring_string(mathml::LETTER_R, true) }</option>
            <option value="modulo">{ mathml::ring_string(mathml::integers_modulo_string("p"), true) }</option>
        </select>
    </Show>

    <Show when=move || op.get().operand_ring_type() == OperandRingType::Euclidean>
        <select
            on:change:target=move |ev| set_euclidean_ring.set(ev.target().value())
            prop:value=move || euclidean_ring.get()
        >
            <option value="integers">{ mathml::ring_string(mathml::LETTER_Z, false) }</option>
            <option value="reals">{ mathml::ring_string(mathml::LETTER_R, true) }</option>
            <option value="modulo">{ mathml::ring_string(mathml::integers_modulo_string("p"), true) }</option>
        </select>
    </Show>

    <Show when=do_select_n >
        <input type="number" min="2"
            prop:placeholder=move || {
                match op.get().operand_ring_type() {
                    OperandRingType::Normal => "n",
                    OperandRingType::Field | OperandRingType::Euclidean => "p",
                }
            }
            on:input:target=move |ev| if let Ok(n_int) = ev.target().value().parse() {
                set_n.set(n_int);
            }
            prop:value=n.get().to_string()
        />
    </Show>

    // <button on:click=move |_| { }>
    //     "Calculate"
    // </button>

    <p>
        { move || {
            calculate(
                op.get(),
                lhs.get(),
                rhs.get(),
                normal_ring.get(),
                field_ring.get(),
                euclidean_ring.get(),
                n.get()
            )
        }}
    </p>

    }
}

fn calculate(
    op: Operation,
    lhs: String,
    rhs: String,
    normal_ring: String,
    field_ring: String,
    euclidean_ring: String,
    n: usize,
) -> impl IntoView {
    match op {
        Operation::Add => match normal_ring.as_str() {
            "reals" => {
                let (lhs, rhs) = match parse(Reals, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs + rhs).into_any()
            }
            "integers" => {
                let (lhs, rhs) = match parse(Integers, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs + rhs).into_any()
            }
            "modulo" => {
                let modulo = IntegersModuloN::new(n);
                let (lhs, rhs) = match parse(modulo, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs + rhs).into_any()
            }
            _ => unreachable!(),
        },
        Operation::Sub => match normal_ring.as_str() {
            "reals" => {
                let (lhs, rhs) = match parse(Reals, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs - rhs).into_any()
            }
            "integers" => {
                let (lhs, rhs) = match parse(Integers, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs - rhs).into_any()
            }
            "modulo" => {
                let modulo = IntegersModuloN::new(n);
                let (lhs, rhs) = match parse(modulo, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs - rhs).into_any()
            }
            _ => unreachable!(),
        },
        Operation::Mul => match normal_ring.as_str() {
            "reals" => {
                let (lhs, rhs) = match parse(Reals, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs * rhs).into_any()
            }
            "integers" => {
                let (lhs, rhs) = match parse(Integers, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs * rhs).into_any()
            }
            "modulo" => {
                let modulo = IntegersModuloN::new(n);
                let (lhs, rhs) = match parse(modulo, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                mathml::render_polynomial(lhs * rhs).into_any()
            }
            _ => unreachable!(),
        },
        Operation::Div => match field_ring.as_str() {
            "reals" => {
                let (lhs, rhs) = match parse(Reals, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                let Some((q, r)) = lhs.polynomial_division(rhs) else {
                    return view! { "Error: right-hand side must not be zero" }.into_any();
                };

                let has_rest = !r.is_zero();
                view! {
                    { mathml::render_polynomial(q) }
                    <Show when=move || has_rest>
                        <br />
                        "REM "
                        {
                            let r = r.clone();
                            mathml::render_polynomial(r)
                        }
                    </Show>
                }
                .into_any()
            }
            "modulo" => {
                let Some(modulo) = IntegersModuloP::new(n) else {
                    return view! { "Error: p must be prime" }.into_any();
                };

                let (lhs, rhs) = match parse(modulo, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                let Some((q, r)) = lhs.polynomial_division(rhs) else {
                    return view! { "Error: right-hand side must not be zero" }.into_any();
                };

                let has_rest = !r.is_zero();
                view! {
                    { mathml::render_polynomial(q) }
                    <Show when=move || has_rest>
                        <br />
                        "REM "
                        {
                            let r = r.clone();
                            mathml::render_polynomial(r)
                        }
                    </Show>
                }
                .into_any()
            }
            _ => unreachable!(),
        },
        Operation::Gcd => match euclidean_ring.as_str() {
            "integers" => {
                let Ok(lhs) = lhs.parse::<isize>() else {
                    return view! { {format!("Error: Couldn't parse integer {lhs}")} }.into_any();
                };
                let Ok(rhs) = rhs.parse::<isize>() else {
                    return view! { {format!("Error: Couldn't parse integer {rhs}")} }.into_any();
                };

                let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean_int(lhs, rhs) else {
                    return view! { "Error: One side must be non-zero" }.into_any();
                };

                view! {
                    { gcd }
                    <br />
                    "WITH s = " { s }
                    " AND t = " { t }
                }.into_any()
            }
            "reals" => {
                let (lhs, rhs) = match parse(Reals, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean(PolynomialRing::new(Reals), lhs, rhs) else {
                    return view! { "Error: One side must be non-zero" }.into_any();
                };

                view! {
                    { mathml::render_polynomial(gcd) }
                    <br />
                    "WITH s = " { mathml::render_polynomial(s) }
                    " AND t = " { mathml::render_polynomial(t) }
                }.into_any()
            }
            "modulo" => {
                let Some(modulo) = IntegersModuloP::new(n) else {
                    return view! { "Error: p must be prime" }.into_any();
                };

                let (lhs, rhs) = match parse(modulo, &lhs, &rhs) {
                    Ok(x) => x,
                    Err(err) => return view! { { err } }.into_any(),
                };

                let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean(PolynomialRing::new(modulo), lhs, rhs) else {
                    return view! { "Error: One side must be non-zero" }.into_any();
                };

                view! {
                    { mathml::render_polynomial(gcd) }
                    <br />
                    "WITH s = " { mathml::render_polynomial(s) }
                    " AND t = " { mathml::render_polynomial(t) }
                }.into_any()
            }
            _ => unreachable!()
        },
    }
}

fn parse<R>(ring: R, lhs: &str, rhs: &str) -> Result<(Polynomial<R>, Polynomial<R>), String>
where
    R: ParsableRing + DisplayRing,
{
    let Some(lhs) = Polynomial::parse(ring, lhs) else {
        return Err(format!("Error: Couldn't parse polynomial {lhs}"));
    };

    let Some(rhs) = Polynomial::parse(ring, rhs) else {
        return Err(format!("Error: Couldn't parse polynomial {rhs}"));
    };

    Ok((lhs, rhs))
}
