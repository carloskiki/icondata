use leptos::*;
use std::hash::Hash;

#[component]
pub fn VirtualView<T, EF, N, KF, K>(
    cx: Scope,
    items: Vec<T>,
    key: KF,
    view: EF,
    /// in rem
    width: f32,
    /// in rem
    height: f32,
    items_per_row: u8,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(Scope, T) -> N + 'static,
    N: IntoView,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static,
{
    let container_style = format!(
        "width: {width}rem;
height: {height}rem;",
        width = width * items_per_row as f32,
        height = (items.len() / items_per_row as usize) as f32 * height
    );

    view! { cx,
        <div class="relative"
            style=container_style
        >
        <For
        each=move || 
        key=key
        view=view
        />
        </div>
    }
}
