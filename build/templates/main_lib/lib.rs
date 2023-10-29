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
{% for short_name in short_names %}
pub use icondata_{{short_name}}::{{short_name|capitalize}}Icon::{self, *};
{%- endfor %}

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
