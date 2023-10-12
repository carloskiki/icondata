use leptos_icons::*;
use once_cell::sync::Lazy;
use strum::{IntoEnumIterator, VariantNames};

pub const NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    [
    AiIcon::VARIANTS,
    BiIcon::VARIANTS,
    BsIcon::VARIANTS,
    CgIcon::VARIANTS,
    ChIcon::VARIANTS,
    FaIcon::VARIANTS,
    FiIcon::VARIANTS,
    HiIcon::VARIANTS,
    IoIcon::VARIANTS,
    LuIcon::VARIANTS,
    OcIcon::VARIANTS,
    RiIcon::VARIANTS,
    SiIcon::VARIANTS,
    TbIcon::VARIANTS,
    TiIcon::VARIANTS,
    VsIcon::VARIANTS,
    WiIcon::VARIANTS,
    
    ]
    .concat()
});
pub static ALL_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| {
    itertools::chain! {
    AiIcon::iter().map(| i | Icon::from(i)),
    BiIcon::iter().map(| i | Icon::from(i)),
    BsIcon::iter().map(| i | Icon::from(i)),
    CgIcon::iter().map(| i | Icon::from(i)),
    ChIcon::iter().map(| i | Icon::from(i)),
    FaIcon::iter().map(| i | Icon::from(i)),
    FiIcon::iter().map(| i | Icon::from(i)),
    HiIcon::iter().map(| i | Icon::from(i)),
    IoIcon::iter().map(| i | Icon::from(i)),
    LuIcon::iter().map(| i | Icon::from(i)),
    OcIcon::iter().map(| i | Icon::from(i)),
    RiIcon::iter().map(| i | Icon::from(i)),
    SiIcon::iter().map(| i | Icon::from(i)),
    TbIcon::iter().map(| i | Icon::from(i)),
    TiIcon::iter().map(| i | Icon::from(i)),
    VsIcon::iter().map(| i | Icon::from(i)),
    WiIcon::iter().map(| i | Icon::from(i)),
    
    }
    .collect()
});