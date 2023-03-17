#[cfg(feature = "CgDigitalocean")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDigitalocean")]
/// *This icon requires the feature* `CgDigitalocean` *to be enabled*.
#[component]
pub fn Digitalocean(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 6C8.68629 6 6 8.68629 6 12H1C1 5.92487 5.92487 1 12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23V18C15.3137 18 18 15.3137 18 12C18 8.68629 15.3137 6 12 6Z" fill="currentColor" /><path d="M7 18V13H12V18H7Z" fill="currentColor" /><path d="M3 18V22H7V18H3Z" fill="currentColor" /><path d="M3 18H1V16H3V18Z" fill="currentColor" /></svg>
   }
}