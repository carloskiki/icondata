#[cfg(feature = "RiMapLinePushpin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapLinePushpin")]
/// *This icon requires the feature* `RiMapLinePushpin` *to be enabled*.
#[component]
pub fn Pushpin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13.828 1.686l8.486 8.486-1.415 1.414-.707-.707-4.242 4.242-.707 3.536-1.415 1.414-4.242-4.243-4.95 4.95-1.414-1.414 4.95-4.95-4.243-4.242 1.414-1.415L8.88 8.05l4.242-4.242-.707-.707 1.414-1.415zm.708 3.536l-4.671 4.67-2.822.565 6.5 6.5.564-2.822 4.671-4.67-4.242-4.243z" /></g></svg>
   }
}