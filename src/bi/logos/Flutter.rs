#[cfg(feature = "BiLogosFlutter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosFlutter")]
/// *This icon requires the feature* `BiLogosFlutter` *to be enabled*.
#[component]
pub fn Flutter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13.9 2.01 3.9 12l3.09 3.09 2.71-2.7L20.09 2l-6.19.01zm.82 14.6 5.39-5.38h-5.93c-.11 0-.26 0-.34.07l-2.23 2.23-3.09 3.07 3.09 3.1 2.15 2.15c.07.07.14.17.26.15h6.07z" /></svg>
   }
}