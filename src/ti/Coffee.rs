#[cfg(feature = "TiCoffee")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiCoffee")]
/// *This icon requires the feature* `TiCoffee` *to be enabled*.
#[component]
pub fn Coffee(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M17 19h-12c-.553 0-1-.447-1-1s.447-1 1-1h12c.553 0 1 .447 1 1s-.447 1-1 1zM17.5 5h-12.5v9c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-2h.5c1.93 0 3.5-1.57 3.5-3.5s-1.57-3.5-3.5-3.5zm-2.5 9h-8v-7h8v7zm2.5-4h-1.5v-3h1.5c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5z" /></g></svg>
   }
}