#[cfg(feature = "OcLgRepoPush")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgRepoPush")]
/// *This icon requires the feature* `OcLgRepoPush` *to be enabled*.
#[component]
pub fn RepoPush(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4.75 0A2.75 2.75 0 0 0 2 2.75v16.5A2.75 2.75 0 0 0 4.75 22h11a.75.75 0 0 0 0-1.5h-11c-.69 0-1.25-.56-1.25-1.25V18A1.5 1.5 0 0 1 5 16.5h7.25a.75.75 0 0 0 0-1.5H5c-.546 0-1.059.146-1.5.401V2.75c0-.69.56-1.25 1.25-1.25H18.5v7a.75.75 0 0 0 1.5 0V.75a.75.75 0 0 0-.75-.75H4.75Z" /><path d="m20 13.903 2.202 2.359a.75.75 0 0 0 1.096-1.024l-3.5-3.75a.75.75 0 0 0-1.096 0l-3.5 3.75a.75.75 0 1 0 1.096 1.024l2.202-2.36v9.348a.75.75 0 0 0 1.5 0v-9.347Z" /></svg>
   }
}