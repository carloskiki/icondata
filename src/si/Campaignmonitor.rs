#[cfg(feature = "SiCampaignmonitor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCampaignmonitor")]
/// *This icon requires the feature* `SiCampaignmonitor` *to be enabled*.
#[component]
pub fn Campaignmonitor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M23.836 4.27c-.29-.413-.86-.515-1.273-.226L.163 19.73c.167.235.437.39.747.39h22.18c.503 0 .91-.41.91-.914V4.78c-.004-.176-.058-.352-.164-.51zm-22.4-.226c-.413-.29-.982-.19-1.272.226-.107.154-.162.332-.164.51v14.45l10.664-8.736-9.227-6.45v-.002z" /></svg>
   }
}