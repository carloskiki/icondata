#[cfg(feature = "HiMdSolidRss")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidRss")]
/// *This icon requires the feature* `HiMdSolidRss` *to be enabled*.
#[component]
pub fn Rss(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M3.75 3C3.33579 3 3 3.33579 3 3.75V4.25C3 4.66421 3.33579 5 3.75 5H4C10.0751 5 15 9.92487 15 16V16.25C15 16.6642 15.3358 17 15.75 17H16.25C16.6642 17 17 16.6642 17 16.25V16C17 8.8203 11.1797 3 4 3H3.75Z" fill="#0F172A" /><path d="M3 8.75C3 8.33579 3.33579 8 3.75 8H4C8.41828 8 12 11.5817 12 16V16.25C12 16.6642 11.6642 17 11.25 17H10.75C10.3358 17 10 16.6642 10 16.25V16C10 12.6863 7.31371 10 4 10H3.75C3.33579 10 3 9.66421 3 9.25V8.75Z" fill="#0F172A" /><path d="M7 15C7 16.1046 6.10457 17 5 17C3.89543 17 3 16.1046 3 15C3 13.8954 3.89543 13 5 13C6.10457 13 7 13.8954 7 15Z" fill="#0F172A" /></svg>
   }
}