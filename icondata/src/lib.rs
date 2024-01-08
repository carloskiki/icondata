//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons libraries from the icondata_* crates.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data
//! used by a component library.
//!
//! [`IconData`]: icondata_core::IconData
//!
//! __Warning!!!:__ the use of `lto = true` in your `Cargo.toml` is _very strongly_
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

pub use icondata_ai::*;
pub use icondata_bi::*;
pub use icondata_bs::*;
pub use icondata_cg::*;
pub use icondata_ch::*;
pub use icondata_fa::*;
pub use icondata_fi::*;
pub use icondata_hi::*;
pub use icondata_im::*;
pub use icondata_io::*;
pub use icondata_lu::*;
pub use icondata_oc::*;
pub use icondata_ri::*;
pub use icondata_si::*;
pub use icondata_tb::*;
pub use icondata_ti::*;
pub use icondata_vs::*;
pub use icondata_wi::*;