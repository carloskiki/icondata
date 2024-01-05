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
{% for short_name in short_names %}
pub use icondata_{{short_name}}::{{short_name|capitalize}}Icon::{self, *};
{%- endfor %}

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
    {% for short_name in short_names -%}
    {{short_name|capitalize}}({{short_name|capitalize}}Icon),
    {% endfor %}
}
impl From<Icon> for icondata_core::IconData {
    fn from(icon: Icon) -> Self {
        match icon {
            {% for short_name in short_names -%}
            Icon::{{short_name|capitalize}}(icon) => icondata_core::IconData::from(icon),
            {% endfor %}
        }
    }
}
{% for short_name in short_names %}
impl From<{{short_name|capitalize}}Icon> for Icon {
    fn from(icon: {{short_name|capitalize}}Icon) -> Self {
        Self::{{short_name|capitalize}}(icon)
    }
}
{% endfor %}
