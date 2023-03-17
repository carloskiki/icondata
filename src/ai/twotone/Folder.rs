#[cfg(feature = "AiTwotoneFolder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiTwotoneFolder")]
/// *This icon requires the feature* `AiTwotoneFolder` *to be enabled*.
#[component]
pub fn Folder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><path fill="#333" d="M880 298.4H521L403.7 186.2a8.15 8.15 0 0 0-5.5-2.2H144c-17.7 0-32 14.3-32 32v592c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V330.4c0-17.7-14.3-32-32-32zM840 768H184V256h188.5l119.6 114.4H840V768z" /><path fill="#E6E6E6" d="M372.5 256H184v512h656V370.4H492.1z" /></svg>
   }
}