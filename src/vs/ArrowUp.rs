#[cfg(feature = "VsArrowUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowUp")]
/// *This icon requires the feature* `VsArrowUp` *to be enabled*.
#[component]
pub fn ArrowUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.854 7l-5-5h-.707l-5 5 .707.707L8 3.561V14h1V3.56l4.146 4.147.708-.707z" /></svg>
   }
}