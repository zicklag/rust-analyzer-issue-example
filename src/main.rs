#![allow(warnings)]
use bones_utils_macros::Deref;

#[derive(Default, Deref)]
struct Example {
    #[deref]
    s: String,
    other: f32,
}

fn main() {
    let e = Example::default();

    let s = e.as_str();

    dbg!(s);
}
