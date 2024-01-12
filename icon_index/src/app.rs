use leptos::*;
use leptos_meta::*;

use crate::icons::*;
use crate::header::*;
use crate::alert::*;
use crate::searchbar::*;

#[derive(Clone, Copy)]
pub struct DarkModeRw(pub RwSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
    let dark_mode = create_rw_signal(get_theme_mode().unwrap_or_default());
    provide_context(DarkModeRw(dark_mode));
    let alert_manager = AlertManager::new();
    provide_context(alert_manager);
    provide_context(SearchContent(create_rw_signal(String::new())));

    create_effect(move |_| {
        set_theme_mode(dark_mode.get())
            .unwrap_or_else(|| log::debug!("could not set theme preference"));
    });

    view! {
    <Html class={move || {
        if dark_mode.get() {
            return "dark".to_string();
        }
        "".to_string()
    }} />
    <Body class="bg-primary text-black dark:bg-primary-dark dark:text-white" />
        <Header />
        <Icons />
        <Alerts alert_manager=alert_manager />
    }
}

fn get_theme_mode() -> Option<bool> {
    window()
        .local_storage()
        .ok()
        .flatten()?
        .get_item("dark_mode")
        .ok()
        .flatten()
        .and_then(|value| match value.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        })
}

/// Returns `Some(())` if operation succeeded, `None` otherwise
fn set_theme_mode(mode: bool) -> Option<()> {
    window()
        .local_storage()
        .ok()
        .flatten()?
        .set_item("dark_mode", &mode.to_string())
        .ok()
}
