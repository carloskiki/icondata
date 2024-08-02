//! This crate provides a collection of icons in the form of SVG data.
//! It re-exports all icon libraries from the icondata_* crates.
//!
//! The [`Icon`] type alias refers to an [`IconData`] struct, which contains the SVG data
//! used by a component library.
//!
//! __Available icons can be searched and seleted [here](https://carloskiki.github.io/icondata/).__
//!
//! [`IconData`]: icondata_core::IconData
//!
//! # Getting Started
//!
//! 1. Add the latest version of this crate to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! icondata = "..."
//! ```
//!
//! 2. Import and use the icons in your code:
//! ```rust
//! let icon = icondata::BsBuildingDown;
//! let icon = icondata::AiAlertFilled;
//! ```
//!
//! __Note:__ importing `icondata::*` will import all icons, which can heavily slow down rust-analyzer.
//! This can be avoided by importing only the icons you need: `use icondata::{..., ...};`, or by
//! using the qualified path as above.
//!
//! If you only need one or a few icon packs, you can enable only the features you need:
//! ```toml
//! [dependencies]
//! icondata = { version = "...", default-features = false, features = ["lu", "bi"] }
//! ```

#[cfg(feature = "ant-design-icons")]
pub use icondata_ai::*;
#[cfg(feature = "box-icons")]
pub use icondata_bi::*;
#[cfg(feature = "bootstrap-icons")]
pub use icondata_bs::*;
#[cfg(feature = "css-gg")]
pub use icondata_cg::*;
#[cfg(feature = "charm")]
pub use icondata_ch::*;
#[cfg(feature = "font-awesome")]
pub use icondata_fa::*;
#[cfg(feature = "feather")]
pub use icondata_fi::*;
#[cfg(feature = "heroicons")]
pub use icondata_hi::*;
#[cfg(feature = "ico-moon-free")]
pub use icondata_im::*;
#[cfg(feature = "ionicons")]
pub use icondata_io::*;
#[cfg(feature = "lucide")]
pub use icondata_lu::*;
#[cfg(feature = "github-octicons")]
pub use icondata_oc::*;
#[cfg(feature = "remix-icon")]
pub use icondata_ri::*;
#[cfg(feature = "simple-icons")]
pub use icondata_si::*;
#[cfg(feature = "tabler-icons")]
pub use icondata_tb::*;
#[cfg(feature = "typicons")]
pub use icondata_ti::*;
#[cfg(feature = "vs-code-icons")]
pub use icondata_vs::*;
#[cfg(feature = "weather-icons")]
pub use icondata_wi::*;

/// An Icon from any of the icondata_* crates.
///
/// The underlying type of this type alias implements many useful traits such as `Eq`, `Hash`,
/// `Serialize`, `Deserialize`, etc. See the [`IconData`](icondata_core::IconData) struct for more information.
pub use icondata_core::Icon;