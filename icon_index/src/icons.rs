use std::{
    cmp::{max, min},
    time::Duration,
};

use icon_index::ICONS;
use leptos::{html::Main, *};
use leptos_icons::Icon;
use web_sys::MouseEvent;

use crate::{
    alert::{Alert, AlertManager},
    searchbar::SearchContent,
};

// TODO: Test overscroll behaivor
#[component]
pub fn Icons() -> impl IntoView {
    let SearchContent(search_content) = use_context::<SearchContent>().unwrap();
    let container_ref: NodeRef<Main> = create_node_ref();
    let (window_size, set_window_size) = create_signal(fetch_window_size());
    let (scroll_pos, set_scroll_pos) = create_signal(0);

    // Set event listener for the resize of the window, to update the number of columns
    window_event_listener(ev::resize, move |_| {
        set_window_size.set(fetch_window_size());
    });
    // Set event listener for the scrolling of the window
    window_event_listener(ev::scroll, move |_| {
        let scroll_y = window().scroll_y().unwrap() as u32;

        set_scroll_pos.set(scroll_y);
    });

    let search_content_memo = create_memo(move |_| {
        search_content.get()
    });
    let filtered_search = Memo::new(move |_| {
        log::debug!("Filtering icons");
        ICONS
            .into_iter()
            .filter_map(|(name, lower_name, icon)| {
                lower_name
                    .contains(&search_content_memo.get())
                    .then(|| (*name, *icon))
            })
            .collect::<Vec<_>>()
    });

    let item_rem_size = 10;
    let base_font = base_font();
    let item_size = item_rem_size * base_font;

    let col_count = Memo::new(move |_| max(window_size.get().0 / item_size, 1));
    let row_count = Memo::new(move |_| window_size.get().1 / item_size);

    let skipped_items = Memo::new(move |_| scroll_pos.get() / item_size * col_count.get());

    let items = create_memo(move |_| {
        filtered_search.with(|icons: &Vec<(&str, icondata::Icon)>| {
            let end = skipped_items.get() + col_count.get() * (row_count.get() + 1);
            let show_range = (min(skipped_items.get() as usize, icons.len()))..(min(end as usize, icons.len()));

            icons[show_range]
                .iter()
                .enumerate()
                .map(move |(pos, icon)| {
                    let top = (skipped_items.get() + pos as u32) / col_count.get() * item_size;
                    let gap_size = (window_size.get().0 - col_count.get() * item_size)
                        / max(col_count.get() - 1, 1);
                    let left = (skipped_items.get() + pos as u32) % col_count.get()
                        * (item_size + gap_size);

                    view! {
                        <IconItem icon=icon.1 name=icon.0 top left />
                    }
                })
                .collect_view()
        })
    });

    // 0. Get rem conversion
    // 1. Get the number of columns
    // 2. Get the number of rows
    // 3. Get the number of icons in window + 2 rows on top and bottom
    // 4. Every icon should have its own pos
    let container_height = move || {
        let item_count = filtered_search.with(|icons| icons.len());
        let height = (item_count as u32 / col_count.get() + 1) * item_size;
        let styles = format!("height: {height}px;");
        styles
    };

    view! {
        <main _ref=container_ref class="relative mx-4"
            // Set the number of columns
            style=container_height
        >
            {items}
        </main>
    }
}

fn fetch_window_size() -> (u32, u32) {
    let html = document().document_element().unwrap();
    let width = html.client_width();
    let height = html.client_height();
    (width as u32, height as u32)
}

// TODO: Error handling
fn base_font() -> u32 {
    let base_font: u32 = window()
        .get_computed_style(&document().body().unwrap())
        .unwrap()
        .unwrap()
        .get_property_value("font-size")
        .expect("could not get base font size")
        .strip_suffix("px")
        .unwrap()
        .parse()
        .unwrap();
    base_font
}

#[component]
pub fn IconItem(icon: icondata::Icon, name: &'static str, top: u32, left: u32) -> impl IntoView {
    let text_size = match name.len() {
        0..=16 => "text-xs",
        17..=22 => "text-[0.6rem]",
        _ => "text-[0.5rem]",
    };

    let alert_manager = use_context::<AlertManager>().unwrap();

    let copy_name = move |_: MouseEvent| {
        let clipboard = window().navigator().clipboard().unwrap();
        let _ = clipboard.write_text(name);

        let alert = Alert {
            text: format!("Copied! {}", name),
        };

        alert_manager.add_alert(alert, Duration::from_secs(2));
    };

    view! {
        <div
            class="rounded-xl border-gray-400 absolute dark:border-gray-200 border-4
            hover:border-secondary hover:dark:border-secondary-dark w-32 h-32
            flex justify-around items-center flex-col group hover:cursor-pointer
            "
            on:click=copy_name
            style=move || format!("top: {top}px; left: {left}px;")
            >
            <Icon icon=icon width="4em" height="4em" class="group-hover:text-emphasis
            group-hover:dark:text-emphasis-dark transition-colors delay-75 duration-200
            ease-in-out" />
            <p class={"line-clamp-1 break-all px-1 ".to_owned() + text_size}>{move || name}</p>
        </div>
    }
}
