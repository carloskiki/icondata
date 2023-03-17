#[cfg(feature = "IoToggleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoToggleSharp")]
/// *This icon requires the feature* `IoToggleSharp` *to be enabled*.
#[component]
pub fn ToggleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M368,112H144a144,144,0,0,0,0,288H368a144,144,0,0,0,0-288Zm0,230a86,86,0,1,1,86-86A85.88,85.88,0,0,1,368,342Z" /></svg>
   }
}