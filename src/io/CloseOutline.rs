#[cfg(feature = "IoCloseOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCloseOutline")]
/// *This icon requires the feature* `IoCloseOutline` *to be enabled*.
#[component]
pub fn CloseOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="368" y1="368" x2="144" y2="144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="368" y1="144" x2="144" y2="368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}