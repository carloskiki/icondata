use leptos_icons::*;
use once_cell::sync::Lazy;
use strum::{IntoEnumIterator, VariantNames};

pub const NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    [
    {% for short_name in short_names -%}
        {{short_name|capitalize}}Icon::VARIANTS,
    {% endfor %}
    ]
    .concat()
});
pub static ALL_ICONS: Lazy<Vec<IconData>> = Lazy::new(|| {
    itertools::chain! {
    {% for short_name in short_names -%}
        {{short_name|capitalize}}Icon::iter().map(| i | IconData::from(i)),
    {% endfor %}
    }
    .collect()
});
