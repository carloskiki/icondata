#[cfg(feature = "IoDownloadSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDownloadSharp")]
/// *This icon requires the feature* `IoDownloadSharp` *to be enabled*.
#[component]
pub fn DownloadSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M272,160V307.37l64-64L358.63,266,256,368.63,153.37,266,176,243.37l64,64V160H92a12,12,0,0,0-12,12V468a12,12,0,0,0,12,12H420a12,12,0,0,0,12-12V172a12,12,0,0,0-12-12Z" /><rect x="240" y="32" width="32" height="128" /></svg>
   }
}