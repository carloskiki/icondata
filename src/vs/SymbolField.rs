#[cfg(feature = "VsSymbolField")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolField")]
/// *This icon requires the feature* `VsSymbolField` *to be enabled*.
#[component]
pub fn SymbolField(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14.45 4.5l-5-2.5h-.9l-7 3.5-.55.89v4.5l.55.9 5 2.5h.9l7-3.5.55-.9v-4.5l-.55-.89zm-8 8.64l-4.5-2.25V7.17l4.5 2v3.97zm.5-4.8L2.29 6.23l6.66-3.34 4.67 2.34-6.67 3.11zm7 1.55l-6.5 3.25V9.21l6.5-3v3.68z" /></svg>
   }
}