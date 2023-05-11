use leptos::*;

use app::*;

mod icons;
mod app;
mod header;
mod searchbar;
mod alert;
// mod virtual_view;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    })
}
