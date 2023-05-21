//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
pub use icondata_core::IconData;
{% for short_name in short_names -%}
#[cfg!(feature = "{{short_name|capitalize}}")]
pub use icondata_{{short_name}}::*;
{%- endfor %}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {

}
impl<'a> icondata_core::IconData<'a> for crate::Icon {
    fn data(self) -> &'a icondata_core::Data {
        match self {
            {% for short_name in short_names -%}
            #[cfg(feature = "{{short_name|capitalize}}")]
            Self::{{short_name|capitalize}}(icon) => icondata_core::IconData::from(icon),
            {%- endfor %}
        }
    }
}
{% for short_name in short_names %}
#[cfg(feature = "{{short_name|capitalize}}")]
impl From<{{short_name|capitalize}}Icon> for Icon {
    fn from(value: {{short_name|capitalize}}Icon) -> Self {
        Self::{{short_name|capitalize}}(value)
    }
}
{% endfor %}
