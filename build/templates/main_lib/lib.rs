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
{% for short_name in short_names %}
pub use icondata_{{short_name}}::*;
{%- endfor %}

/// An Icon from any of the icondata_* crates.
///
/// The underlying type of this type alias implements many useful traits such as `Eq`, `Hash`,
/// `Serialize`, `Deserialize`, etc. See the [`IconData`](icondata_core::IconData) struct for more information.
pub use icondata_core::Icon;
