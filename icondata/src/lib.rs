//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons from the icondata_* crates.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data
//! used by a component library.
//!
//! [`IconData`]: icondata_core::IconData
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
//! 2. Set the `ICONDATA_MANIFEST_DIR` environment variable to the path of your `Icondata.toml` file:
//! This can be done in multiple ways, but the recommended way is to set it in `.cargo/config.toml`
//! (This assumes that your `Icondata.toml` file is in the root of your project):
//! ```toml
//! [env]
//! ICONDATA_MANIFEST_DIR = { value = "", relative = true }
//! ```
//!
//! 3. In order to not clog the namespace and to minimize binary sizes, Icons are imported
//!    individually by enabling them in the `Icondata.toml` file. Please refer to the documentation
//!    of the [__Icondata Manifest__](#icondata-manifest) for more information.
//!
//!    Create an `Icondata.toml` file in the root of your project, and include icons.
//!
//!    Icons can be searched with the [__icon index__](https://carlosted.github.io/icondata/).
//!    Here is a basic example:
//! ```toml
//! icons = ["AiAlertFilled", "BsBuildingDown"]
//! ```
//!
//! 4. Import and use the icons in your code:
//! ```rust
//! use icondata::*;
//!
//! let icon = BsBuildingDown;
//! let icon = AiAlertFilled;
//! ```
//!
//! # Icondata Manifest
//!
//! The Icondata manifest is a TOML file named `Icondata.toml` that is usually 
//! placed in the root of your project/workspace (but can be placed anyhwere).
//! The format of this file is currently very simple, but may be extended in the future:
//!
//! ```toml
//! include-all = false # If set to `true`, all icons will be included
//!                     # (takes precedence over `icons` field).
//!                     # Can be useful for testing many icons,
//!                     # but won't make `rust-analyzer` happy. 
//!                     # It is very important to set `lto = true` in your
//!                     # `Cargo.toml` when using this feature,
//!                     # otherwise your binary size will be huge.
//!
//! icons = ["MyIcon1", "MyIcon2", ...] # Array of icons include.
//! ```
//!
//! ## Environment Variables
//!
//! - `ICONDATA_MANIFEST_DIR`: Required to use the Icondata manifest. It is recommended 
//! to set it in the `.cargo/config.toml` file of your project:
//! ```toml
//! [env]
//! ICONDATA_MANIFEST_DIR = { value = "path/to/dir", relative = true }
//! ```
//!
//! - `ICONDATA_INCLUDE_ALL`: If set to `true` or something other than 0, all icons will be included. Takes precedence over
//! the Icondata manifest.
//! 

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
/// This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
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
