use leptos_icons::*;
use enum_iterator::all;

fn main() {
    println!("Hello, world!");
}

fn all_icons() -> Vec<impl Into<icondata_core::IconData>>  {
    vec![
        all::<BsIcon>()
    ]
}
