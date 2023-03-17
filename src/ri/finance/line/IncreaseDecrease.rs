#[cfg(feature = "RiFinanceLineIncreaseDecrease")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceLineIncreaseDecrease")]
/// *This icon requires the feature* `RiFinanceLineIncreaseDecrease` *to be enabled*.
#[component]
pub fn IncreaseDecrease(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm1 2v14h16V5H4zm5 6h2v2H9v2H7v-2H5v-2h2V9h2v2zm4 0h6v2h-6v-2z" /></g></svg>
   }
}