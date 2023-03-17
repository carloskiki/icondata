#[cfg(feature = "FiBluetooth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiBluetooth")]
/// *This icon requires the feature* `FiBluetooth` *to be enabled*.
#[component]
pub fn Bluetooth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5" /></svg>
   }
}