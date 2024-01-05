//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons libraries from the icondata_* crates.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data
//! used by a component library.
//!
//! [`IconData`]: icondata_core::IconData
//!
//! ___Warning!!!:___ the use of `lto = true` in your `Cargo.toml` is _very __strongly___
//! encouraged. Otherwise, the size of your binary may become very large from unused icons.
//!
//! # Getting Started
//!
//! 1. Add the latest version of this crate to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! # .... Other dependencies ....
//! icondata = "..."
//! # .... Other dependencies ....
//! ```
//!
//! 2. Import and use the icons in your code:
//! ```rust
//! let icon = icondata::BsBuildingDown;
//! let icon = icondata::AiAlertFilled;
//! ```
//! 
//! __Note:__ importing `icondata::*` will import all icons, which can slow down rust-analyzer.
//! This can be avoided by importing only the icons you need: `use icondata::{..., ...};`, or by
//! using the qualified path as above.

pub use icondata_ai::AiIcon::{self, *};
pub use icondata_bi::BiIcon::{self, *};
pub use icondata_bs::BsIcon::{self, *};
pub use icondata_cg::CgIcon::{self, *};
pub use icondata_ch::ChIcon::{self, *};
pub use icondata_fa::FaIcon::{self, *};
pub use icondata_fi::FiIcon::{self, *};
pub use icondata_hi::HiIcon::{self, *};
pub use icondata_im::ImIcon::{self, *};
pub use icondata_io::IoIcon::{self, *};
pub use icondata_lu::LuIcon::{self, *};
pub use icondata_oc::OcIcon::{self, *};
pub use icondata_ri::RiIcon::{self, *};
pub use icondata_si::SiIcon::{self, *};
pub use icondata_tb::TbIcon::{self, *};
pub use icondata_ti::TiIcon::{self, *};
pub use icondata_vs::VsIcon::{self, *};
pub use icondata_wi::WiIcon::{self, *};

/// The  enum regrouping all Icon Libraries.
///
/// This enum contains all icons from icondata_* libraries, and
/// implements [`From`] for [`IconData`], so it can be converted into an [`IconData`] struct.
/// It also implements [`From`] for any icon in any icondata_* library. 
///
/// This enum is marked as non_exhaustive as new icon libraries may be added in the future.
///
/// [`IconData`]: icondata_core::IconData
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {
    Ai(AiIcon),
    Bi(BiIcon),
    Bs(BsIcon),
    Cg(CgIcon),
    Ch(ChIcon),
    Fa(FaIcon),
    Fi(FiIcon),
    Hi(HiIcon),
    Im(ImIcon),
    Io(IoIcon),
    Lu(LuIcon),
    Oc(OcIcon),
    Ri(RiIcon),
    Si(SiIcon),
    Tb(TbIcon),
    Ti(TiIcon),
    Vs(VsIcon),
    Wi(WiIcon),
    
}
impl From<Icon> for icondata_core::IconData {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Ai(icon) => icondata_core::IconData::from(icon),
            Icon::Bi(icon) => icondata_core::IconData::from(icon),
            Icon::Bs(icon) => icondata_core::IconData::from(icon),
            Icon::Cg(icon) => icondata_core::IconData::from(icon),
            Icon::Ch(icon) => icondata_core::IconData::from(icon),
            Icon::Fa(icon) => icondata_core::IconData::from(icon),
            Icon::Fi(icon) => icondata_core::IconData::from(icon),
            Icon::Hi(icon) => icondata_core::IconData::from(icon),
            Icon::Im(icon) => icondata_core::IconData::from(icon),
            Icon::Io(icon) => icondata_core::IconData::from(icon),
            Icon::Lu(icon) => icondata_core::IconData::from(icon),
            Icon::Oc(icon) => icondata_core::IconData::from(icon),
            Icon::Ri(icon) => icondata_core::IconData::from(icon),
            Icon::Si(icon) => icondata_core::IconData::from(icon),
            Icon::Tb(icon) => icondata_core::IconData::from(icon),
            Icon::Ti(icon) => icondata_core::IconData::from(icon),
            Icon::Vs(icon) => icondata_core::IconData::from(icon),
            Icon::Wi(icon) => icondata_core::IconData::from(icon),
            
        }
    }
}

impl From<AiIcon> for Icon {
    fn from(icon: AiIcon) -> Self {
        Self::Ai(icon)
    }
}

impl From<BiIcon> for Icon {
    fn from(icon: BiIcon) -> Self {
        Self::Bi(icon)
    }
}

impl From<BsIcon> for Icon {
    fn from(icon: BsIcon) -> Self {
        Self::Bs(icon)
    }
}

impl From<CgIcon> for Icon {
    fn from(icon: CgIcon) -> Self {
        Self::Cg(icon)
    }
}

impl From<ChIcon> for Icon {
    fn from(icon: ChIcon) -> Self {
        Self::Ch(icon)
    }
}

impl From<FaIcon> for Icon {
    fn from(icon: FaIcon) -> Self {
        Self::Fa(icon)
    }
}

impl From<FiIcon> for Icon {
    fn from(icon: FiIcon) -> Self {
        Self::Fi(icon)
    }
}

impl From<HiIcon> for Icon {
    fn from(icon: HiIcon) -> Self {
        Self::Hi(icon)
    }
}

impl From<ImIcon> for Icon {
    fn from(icon: ImIcon) -> Self {
        Self::Im(icon)
    }
}

impl From<IoIcon> for Icon {
    fn from(icon: IoIcon) -> Self {
        Self::Io(icon)
    }
}

impl From<LuIcon> for Icon {
    fn from(icon: LuIcon) -> Self {
        Self::Lu(icon)
    }
}

impl From<OcIcon> for Icon {
    fn from(icon: OcIcon) -> Self {
        Self::Oc(icon)
    }
}

impl From<RiIcon> for Icon {
    fn from(icon: RiIcon) -> Self {
        Self::Ri(icon)
    }
}

impl From<SiIcon> for Icon {
    fn from(icon: SiIcon) -> Self {
        Self::Si(icon)
    }
}

impl From<TbIcon> for Icon {
    fn from(icon: TbIcon) -> Self {
        Self::Tb(icon)
    }
}

impl From<TiIcon> for Icon {
    fn from(icon: TiIcon) -> Self {
        Self::Ti(icon)
    }
}

impl From<VsIcon> for Icon {
    fn from(icon: VsIcon) -> Self {
        Self::Vs(icon)
    }
}

impl From<WiIcon> for Icon {
    fn from(icon: WiIcon) -> Self {
        Self::Wi(icon)
    }
}
