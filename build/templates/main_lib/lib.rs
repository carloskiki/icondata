//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons from the icondata_* crates.
//!
//! This crate is meant to be used to build component libraries for web frameworks.
//! To do so, an [`Icon`] enum is provided, which can be used to select any icon from the
//! collection. This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data.
//!
//!
{% for short_name in short_names %}
#[cfg(feature = "{{short_name|capitalize}}")]
pub use icondata_{{short_name}}::{{short_name|capitalize}}Icon::{self, *};
{%- endfor %}

/// The main enum to select an icon. This enum contains all icons from icondata_* libraries, and
/// implements [`From`] for [`IconData`], so it can be converted into an [`IconData`] struct.
/// It also implements [`From`] for any icon in any icondata_* library.
///
/// This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {
    {% for short_name in short_names -%}
    #[cfg(feature = "{{short_name|capitalize}}")]
    {{short_name|capitalize}}({{short_name|capitalize}}Icon),
    {% endfor %}
}
impl From<Icon> for icondata_core::IconData {
    fn from(icon: Icon) -> Self {
        match icon {
            {% for short_name in short_names -%}
            #[cfg(feature = "{{short_name|capitalize}}")]
            Icon::{{short_name|capitalize}}(icon) => icondata_core::IconData::from(icon),
            {% endfor %}
        }
    }
}
{% for short_name in short_names %}
#[cfg(feature = "{{short_name|capitalize}}")]
impl From<{{short_name|capitalize}}Icon> for Icon {
    fn from(icon: {{short_name|capitalize}}Icon) -> Self {
        Self::{{short_name|capitalize}}(icon)
    }
}
{% endfor %}
