#[cfg(feature = "IoGitCompareOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGitCompareOutline")]
/// *This icon requires the feature* `IoGitCompareOutline` *to be enabled*.
#[component]
pub fn GitCompareOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="304 160 240 96 304 32" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="207 352 271 416 207 480" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="112" cy="96" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="400" cy="416" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M256,96h84a60,60,0,0,1,60,60V368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M255,416H171a60,60,0,0,1-60-60V144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}