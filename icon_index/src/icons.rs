use std::time::Duration;

use icon_index::all_icons;
use leptos::{leptos_dom::console_log, *};
use leptos_icons::{Icon, IconData, IconProps};
use web_sys::MouseEvent;

use crate::{alert::{AlertManager, Alert}, searchbar::SearchContent};

#[component]
pub fn Icons(cx: Scope) -> impl IntoView {
    let SearchContent(search_content) = use_context::<SearchContent>(cx).unwrap();

    view! { cx,
        <main>
            <For
            each=move || all_icons().filter(|(feat_name, _)| feat_name.starts_with("Fa"))
            key=|(feat_name, _)| *feat_name
            view=move |cx, (feat_name, icon)| {
                view!{ cx,
                <IconItem icon=icon feat_name=feat_name />
                }
            }
            />
        </main>
    }
}

#[component]
pub fn IconItem(cx: Scope, icon: IconData, feat_name: &'static str) -> impl IntoView {
    let text_size = match feat_name.len() {
        0..=16 => "text-xs",
        17..=22 => "text-[0.6rem]",
        _ => "text-[0.5rem]",
    };

    let alert_manager = use_context::<AlertManager>(cx).unwrap();

    let copy_name = move |_: MouseEvent| {
        let clipboard = window().navigator().clipboard().unwrap();
        let _ = clipboard.write_text(feat_name);

        let alert = Alert {
            text: format!("Copied! {}", feat_name)
        };

        alert_manager.add_alert(alert, Duration::from_secs(2));
    };

    view! { cx,
        <div
            class="rounded-xl border-gray-400 dark:border-gray-200 border-4
            hover:border-secondary hover:dark:border-secondary-dark m-4 w-32 h-32
            flex justify-around items-center flex-col group hover:cursor-pointer
            "
            on:click=copy_name
            >
            <Icon icon=icon width="4em" height="4em" class="group-hover:text-emphasis
            group-hover:dark:text-emphasis-dark transition-colors delay-75 duration-200
            ease-in-out" />
            <p class=text_size>{move || feat_name}</p>
        </div>
    }
}
