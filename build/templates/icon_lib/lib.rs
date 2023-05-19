//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
//! The enum implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
//!
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub enum {{short_name|capitalize}}Icon {
    {% for icon in icons %}
    {% let feat = icon.feature.name %}
    #[cfg(feature = "{{feat|capitalize}}")]
    {{feat}},
    {% endfor %}
}

{% for icon in icons %}
{% let attributes = icon.svg.svg_attributes %}
const {{icon.feature.name|shouty_snake_case}}: icondata_core::IconData = icondata_core::IconData {
    style: {{ "{:?}"|format(attributes.style) }},
    x: {{ "{:?}"|format(attributes.x) }},
    y: {{ "{:?}"|format(attributes.y) }},
    width: {{ "{:?}"|format(attributes.width) }},
    height: {{ "{:?}"|format(attributes.height) }},
    view_box: {{ "{:?}"|format(attributes.view_box) }},
    stroke_linecap: {{ "{:?}"|format(attributes.stroke_linecap) }},
    stroke_linejoin: {{ "{:?}"|format(attributes.stroke_linejoin) }},
    stroke_width: {{ "{:?}"|format(attributes.stroke_width) }},
    stroke: {{ "{:?}"|format(attributes.stroke) }},
    fill: {{ "{:?}"|format(attributes.fill) }},
    data: {{ attibutes.data }}
}
{% endfor %}

impl From<{{short_name|capitalize}}Icon> for icondata_core::IconData {
    fn from(icon: {{short_name|capitalize}}Icon) -> icondata_core::IconData {
        match icon {
            {% for icon in icons %}
            {% let feat = icon.feature.name %}
            #[cfg(feature = "{{feat}}")]
            LuIcon::{{feat}} => {{feat|shouty_snake_case}},
            {% endfor %}
        }
    }
}
