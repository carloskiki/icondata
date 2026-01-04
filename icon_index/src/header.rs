use leptos::{prelude::*, ev::MouseEvent};
use leptos_icons::Icon;
use web_sys::{HtmlHeadingElement, ScrollBehavior, ScrollToOptions};

use crate::searchbar::*;
use crate::DarkModeRw;

#[component]
pub fn Header() -> impl IntoView {
    view! {
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
pub fn HeaderLogo() -> impl IntoView {
    let (pos, set_pos) = signal((0, 0));
    let DarkModeRw(dark_mode) = use_context::<DarkModeRw>().unwrap();

    let logo_animation = move |ev: MouseEvent| {
        let target = event_target::<HtmlHeadingElement>(&ev);
        let dom_rect: web_sys::DomRect = target.get_bounding_client_rect();

        let x = (ev.client_x() as f64 - dom_rect.left()) as i32;
        let y = (ev.client_y() as f64 - dom_rect.top()) as i32;

        set_pos.set((x, y));
    };

    let logo_style = move || {
        let (gradient_suffix, text_color) = match dark_mode.get() {
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
            pos.get().0,
            pos.get().1
        )
    };

    let scroll_to_top = move |_: MouseEvent| {
        let options = ScrollToOptions::new();
        options.set_top(0f64);
        options.set_behavior(ScrollBehavior::Smooth);

        window().scroll_to_with_scroll_to_options(&options);
    };

    view! {
            <h1
            on:mousemove=logo_animation
            on:click=scroll_to_top
            style=logo_style
            class="font-extrabold text-xl sm:text-2xl p-2 cursor-pointer hover:text-transparent transition-colors duration-500 ease-in-out"
            >"Icondata"</h1>
    }
}

#[component]
pub fn ThemeButton() -> impl IntoView {
    let DarkModeRw(dark_mode) = use_context::<DarkModeRw>().unwrap();
    let icon_size = "1.75rem";
    let trigger_theme = move |_| {
        dark_mode.update(|dark| *dark = !*dark);
    };

    let icon = Signal::derive(move || {
        if dark_mode.get() {
            icondata::BsSun
        } else {
            icondata::BsMoonStars
        }
    });

    view! {
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
         <Icon icon width=icon_size height=icon_size />
        </div>
    }
}
