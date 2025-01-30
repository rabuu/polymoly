mod mathml;

use std::fmt::Display;

use leptos::prelude::*;

use polymoly::polynomial::Polynomial;
use polymoly::ring::Reals;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (op, set_op) = signal(Operation::Add);

    let (lhs, set_lhs) = signal(String::new());
    let (rhs, set_rhs) = signal(String::new());

    let (normal_ring, set_normal_ring) = signal(String::from("real"));
    let (field_ring, set_field_ring) = signal(String::from("real"));

    let (n, set_n) = signal(2_usize);
    let do_select_n = move || if op.get().normal_ring() {
        normal_ring.get().as_str() == "zmod"
    } else if op.get().field_ring() {
        field_ring.get().as_str() == "zmod"
    } else {
        false
    };

    let (output, set_output) = signal(None);

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
        // <input type="radio" name="operation" value="gcd" id="op:gcd" on:input:target=move |ev| set_op.set(ev.target().value()) />
        // <label for="op:mul">GCD</label>
    </div>

    <input type="text"
        prop:placeholder="Left-hand side"
        bind:value=(lhs, set_lhs)
    />
    <input type="text"
        prop:placeholder="Right-hand side"
        bind:value=(rhs, set_rhs)
    />

    <Show when=move || op.get().normal_ring()>
        <select
            on:change:target=move |ev| set_normal_ring.set(ev.target().value())
            prop:value=move || normal_ring.get()
        >
            <option value="real">{ mathml::ring(mathml::LETTER_R, true) }</option>
            <option value="integer">{ mathml::ring(mathml::LETTER_Z, true) }</option>
            <option value="zmod">{ mathml::ring(mathml::zmod_string("n"), true) }</option>
        </select>
    </Show>

    <Show when=move || op.get().field_ring()>
        <select
            on:change:target=move |ev| set_field_ring.set(ev.target().value())
            prop:value=move || field_ring.get()
        >
            <option value="real">{ mathml::ring(mathml::LETTER_R, true) }</option>
            <option value="zmod">{ mathml::ring(mathml::zmod_string("p"), true) }</option>
        </select>
    </Show>

    <Show when=do_select_n >
        <input type="number" min="2"
            prop:placeholder="n"
            on:input:target=move |ev| if let Ok(n_int) = ev.target().value().parse() {
                set_n.set(n_int);
            }
            prop:value=n.get().to_string()
        />
    </Show>

    <button on:click=move |_| set_output.set(calculate(op.get(), lhs.get(), rhs.get(), normal_ring.get(), field_ring.get(), n.get()))>
        "Calculate"
    </button>

    // <p> { output } </p>
    }
}

fn calculate(op: Operation, lhs: String, rhs: String, normal_ring: String, field_ring: String, n: usize) -> Option<impl IntoView> {
    Some(view! { hallo })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    // Gcd,
}

impl Operation {
    fn normal_ring(&self) -> bool {
        matches!(self, Operation::Add | Operation::Sub | Operation::Mul)
    }

    fn field_ring(&self) -> bool {
        matches!(self, Operation::Div)
    }
}

impl From<String> for Operation {
    fn from(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            _ => unreachable!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Sub => write!(f, "sub"),
            Operation::Mul => write!(f, "mul"),
            Operation::Div => write!(f, "div"),
        }
    }
}


fn parse(lhs: &str, rhs: &str) -> Result<(Polynomial<Reals>, Polynomial<Reals>), String> {
    let Some(lhs) = Polynomial::parse(Reals, lhs) else {
        return Err(format!("Couldn't parse polynomial {lhs}"));
    };

    let Some(rhs) = Polynomial::parse(Reals, rhs) else {
        return Err(format!("Couldn't parse polynomial {rhs}"));
    };

    Ok((lhs, rhs))
}
