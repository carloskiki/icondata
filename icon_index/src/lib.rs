use leptos_icons::*;
use strum::{IntoEnumIterator, VariantNames};

pub fn all_icons() -> impl Iterator<Item = (&'static str, IconData)> {
    itertools::chain! {
        AiIcon::iter().zip(AiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        FaIcon::iter().zip(FaIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        WiIcon::iter().zip(WiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        FiIcon::iter().zip(FiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        VsIcon::iter().zip(VsIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        BsIcon::iter().zip(BsIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        BiIcon::iter().zip(BiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        ImIcon::iter().zip(ImIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        IoIcon::iter().zip(IoIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        RiIcon::iter().zip(RiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        SiIcon::iter().zip(SiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        TiIcon::iter().zip(TiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        HiIcon::iter().zip(HiIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        CgIcon::iter().zip(CgIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        TbIcon::iter().zip(TbIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        OcIcon::iter().zip(OcIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i))),
        LuIcon::iter().zip(LuIcon::VARIANTS).map(| (i, n) | (* n, IconData::from(i)))
    }
}
