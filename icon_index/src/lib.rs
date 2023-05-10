use once_cell::sync::Lazy;
use std::collections::BTreeMap;

use leptos_icons::*;
use strum::{IntoEnumIterator, VariantNames};

pub static NAMES: &'static [&'static str] = FaIcon::VARIANTS;

pub static ALL_ICONS: Lazy<Vec<IconData>> = Lazy::new(|| {
    FaIcon::iter().map(|i| IconData::from(i)).collect()
});
