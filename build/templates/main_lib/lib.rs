//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
pub use icondata_core::IconData;
{% for lib_short in lib_names %}
#[cfg!(feature = "{{lib_short|capitalize}}")]
pub use icondata_{{short_name}}::*;
{% endfor %}

pub enum Icon {

}
