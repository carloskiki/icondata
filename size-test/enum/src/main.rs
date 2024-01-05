use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let my_icon: icondata_core::IconData =
        icondata_core::IconData::from(icondata::AiAlipayCircleOutlined);
    let other_icon = icondata_core::IconData::from(icondata::AiAppstoreOutlined);

    mount_to_body(move || {
        view! {
            <div>
                <p>"Here is the data of a single icon:"</p>
                <p>{format!("{my_icon:?}")}</p>
                <p>{format!("{other_icon:?}")}</p>
            </div>
        }
    })
}
