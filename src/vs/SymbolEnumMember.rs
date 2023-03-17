#[cfg(feature = "VsSymbolEnumMember")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolEnumMember")]
/// *This icon requires the feature* `VsSymbolEnumMember` *to be enabled*.
#[component]
pub fn SymbolEnumMember(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7 3l1-1h6l1 1v5l-1 1h-4V8h4V3H8v3H7V3zm2 6V8L8 7H2L1 8v5l1 1h6l1-1V9zM8 8v5H2V8h6zm1.414-1L9 6.586V6h4v1H9.414zM9 4h4v1H9V4zm-2 6H3v1h4v-1z" /></svg>
   }
}