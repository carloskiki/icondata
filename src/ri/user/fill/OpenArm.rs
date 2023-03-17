#[cfg(feature = "RiUserFillOpenArm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiUserFillOpenArm")]
/// *This icon requires the feature* `RiUserFillOpenArm` *to be enabled*.
#[component]
pub fn OpenArm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 12a5 5 0 1 1 0-10 5 5 0 0 1 0 10zm6 5v5h-2v-5c0-4.451 2.644-8.285 6.447-10.016l.828 1.82A9.002 9.002 0 0 0 18 17zM8 17v5H6v-5A9.002 9.002 0 0 0 .725 8.805l.828-1.821A11.002 11.002 0 0 1 8 17z" /></g></svg>
   }
}