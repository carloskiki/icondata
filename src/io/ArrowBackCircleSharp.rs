#[cfg(feature = "IoArrowBackCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowBackCircleSharp")]
/// *This icon requires the feature* `IoArrowBackCircleSharp` *to be enabled*.
#[component]
pub fn ArrowBackCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M48,256c0,114.87,93.13,208,208,208s208-93.13,208-208S370.87,48,256,48,48,141.13,48,256Zm224-80.09L208.42,240H358v32H208.42L272,336.09,249.3,358.63,147.46,256,249.3,153.37Z" /></svg>
   }
}