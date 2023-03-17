#[cfg(feature = "FiBell")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiBell")]
/// *This icon requires the feature* `FiBell` *to be enabled*.
#[component]
pub fn Bell(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" /><path d="M13.73 21a2 2 0 0 1-3.46 0" /></svg>
   }
}