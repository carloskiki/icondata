#[cfg(feature = "RiDocumentLineBill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineBill")]
/// *This icon requires the feature* `RiDocumentLineBill` *to be enabled*.
#[component]
pub fn Bill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 22H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1zm-1-2V4H5v16h14zM8 9h8v2H8V9zm0 4h8v2H8v-2z" /></g></svg>
   }
}