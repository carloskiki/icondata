#[cfg(feature = "BiRegularSortUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSortUp")]
/// *This icon requires the feature* `BiRegularSortUp` *to be enabled*.
#[component]
pub fn SortUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 9h9v2h-9zm0 4h7v2h-7zm0-8h11v2H11zm0 12h5v2h-5zm-6 3h2V8h3L6 4 2 8h3z" /></svg>
   }
}