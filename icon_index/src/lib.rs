use leptos_icons::*;
use strum::{IntoEnumIterator, VariantNames};
use once_cell::sync::Lazy;

pub const NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    [
        AiIcon::VARIANTS,
        FaIcon::VARIANTS,
        WiIcon::VARIANTS,
        FiIcon::VARIANTS,
        VsIcon::VARIANTS,
        BsIcon::VARIANTS,
        BiIcon::VARIANTS,
        ImIcon::VARIANTS,
        IoIcon::VARIANTS,
        RiIcon::VARIANTS,
        SiIcon::VARIANTS,
        TiIcon::VARIANTS,
        HiIcon::VARIANTS,
        CgIcon::VARIANTS,
        TbIcon::VARIANTS,
        OcIcon::VARIANTS,
        LuIcon::VARIANTS,
    ]
        .concat()
});
pub static ALL_ICONS: Lazy<Vec<IconData>> = Lazy::new(|| {
    itertools::chain! {
        AiIcon::iter().map(| i | IconData::from(i)), FaIcon::iter().map(| i |
        IconData::from(i)), WiIcon::iter().map(| i | IconData::from(i)), FiIcon::iter()
        .map(| i | IconData::from(i)), VsIcon::iter().map(| i | IconData::from(i)),
        BsIcon::iter().map(| i | IconData::from(i)), BiIcon::iter().map(| i |
        IconData::from(i)), ImIcon::iter().map(| i | IconData::from(i)), IoIcon::iter()
        .map(| i | IconData::from(i)), RiIcon::iter().map(| i | IconData::from(i)),
        SiIcon::iter().map(| i | IconData::from(i)), TiIcon::iter().map(| i |
        IconData::from(i)), HiIcon::iter().map(| i | IconData::from(i)), CgIcon::iter()
        .map(| i | IconData::from(i)), TbIcon::iter().map(| i | IconData::from(i)),
        OcIcon::iter().map(| i | IconData::from(i)), LuIcon::iter().map(| i |
        IconData::from(i))
    }
        .collect()
});
