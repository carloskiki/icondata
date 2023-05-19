use leptos_icons::*;
use strum::{IntoEnumIterator, VariantNames};
use once_cell::sync::Lazy;

pub const NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    [
    {% for short_name in short_names_cap %}
        {{short_name}}Icon::VARIANTS,
    {% endfor %}
    ].concat()
});
pub static ALL_ICONS: Lazy<Vec<IconData>> = Lazy::new(|| {
    itertools::chain! {
    {% for short_name in short_names_cap %}
        {{short_name}}Icon::iter().map(| i | IconData::from(i)),
    {% endfor %}
    }.collect()
});
