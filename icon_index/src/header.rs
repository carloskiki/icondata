use leptos::{ev::MouseEvent, *};
use leptos_icons::{BsIcon, Icon};
use web_sys::{HtmlHeadingElement, ScrollToOptions, ScrollBehavior};

use crate::searchbar::*;
use crate::DarkModeRw;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
    <nav class="sticky top-0 left-0 w-screen flex justify-between px-8 sm:px-16 p-6 items-center bg-primary dark:bg-primary-dark z-50">
        <HeaderLogo />
        <div class="flex space-x-6 items-center">
            <SearchBar />
            <a href="https://github.com/Carlosted/icondata" class="cursor-pointer hover:text-emphasis dark:hover:text-emphasis-dark">"Docs"</a>
            <ThemeButton />
        </div>
    </nav>
    }
}

#[component]
pub fn HeaderLogo(cx: Scope) -> impl IntoView {
    let (pos, set_pos) = create_signal(cx, (0, 0));
    let DarkModeRw(dark_mode) = use_context::<DarkModeRw>(cx).unwrap();

    let logo_animation = move |ev: MouseEvent| {
        let target = event_target::<HtmlHeadingElement>(&ev);
        let dom_rect: web_sys::DomRect = target.get_bounding_client_rect();

        let x = (ev.client_x() as f64 - dom_rect.left()) as i32;
        let y = (ev.client_y() as f64 - dom_rect.top()) as i32;

        set_pos((x, y));
    };

    let logo_style = move || {
        let (gradient_suffix, text_color) = match dark_mode() {
            true => ("-dark", "white"),
            false => ("", "gray"),
        };

        format!(
            "background: radial-gradient(
                200px circle at {}px {}px,
                rgb(var(--color-secondary{gradient_suffix})),
                {text_color} 90%
            );
            -webkit-background-clip: text;
            background-clip: text;
            ",
            pos().0,
            pos().1
        )
    };

    let scroll_to_top = move |_: MouseEvent| {
        let mut options = ScrollToOptions::new();
        options.top(0f64).behavior(ScrollBehavior::Smooth);

        window().scroll_to_with_scroll_to_options(&options);
    };

    view! { cx,
            <h1
            on:mousemove=logo_animation
            on:click=scroll_to_top
            style=logo_style
            class="font-extrabold text-xl sm:text-2xl p-2 cursor-pointer hover:text-transparent transition-colors duration-500 ease-in-out"
            >"Icondata"</h1>
    }
}

#[component]
pub fn ThemeButton(cx: Scope) -> impl IntoView {
    let DarkModeRw(dark_mode) = use_context::<DarkModeRw>(cx).unwrap();
    let icon_size = "1.75rem";
    let trigger_theme = move |_| {
        dark_mode.update(|dark| *dark = !*dark);
    };

    let icon = move || {
        if dark_mode() {
            view! { cx, <Icon icon=BsIcon::BsSun width=icon_size height=icon_size /> }
        } else {
            view! { cx, <Icon icon=BsIcon::BsMoonStars width=icon_size height=icon_size /> }
        }
    };

    view! { cx,
        <div
            on:click=trigger_theme
            class="p-2 rounded-lg border-2 border-secondary
            transition-colors duration-500 ease-in-out
                dark:border-secondary-dark
                hover:bg-secondary-dark
                hover:text-white
                hover:border-primary-dark
                hover:dark:bg-secondary
                hover:dark:text-black
                hover:dark:border-primary
                "
        >
            {icon}
        </div>
    }
}
