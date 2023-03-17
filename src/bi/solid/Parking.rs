#[cfg(feature = "BiSolidParking")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidParking")]
/// *This icon requires the feature* `BiSolidParking` *to be enabled*.
#[component]
pub fn Parking(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13.5 3H5v18h4v-5h4.5c3.584 0 6.5-2.916 6.5-6.5S17.084 3 13.5 3zm0 9H9V7h4.5C14.879 7 16 8.121 16 9.5S14.879 12 13.5 12z" /></svg>
   }
}