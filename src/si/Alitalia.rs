#[cfg(feature = "SiAlitalia")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAlitalia")]
/// *This icon requires the feature* `SiAlitalia` *to be enabled*.
#[component]
pub fn Alitalia(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M5.429 20.551H0l14.257-14.87c1.622-1.765 2.878-2.232 4.686-2.232H24L21.602 20.55h-4.17L19.49 5.907M15.7 20.551l1.384-9.842-9.457 9.842Z" /></svg>
   }
}