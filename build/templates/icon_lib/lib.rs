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
    {% for icon in icons -%}
    {% let feat = icon.feature.name() %}
    #[cfg(feature = "{{feat|capitalize}}")]
    {{feat}},
    {%- endfor %}
}

{% for icon in icons -%}
{% let attributes = icon.svg.svg_attributes() %}
const {{ icon.feature.name()|shouty_snake_case }}: icondata_core::IconData = icondata_core::IconData {
    style: {{ attributes.style|attribute_value }},
    x: {{ attributes.x|attribute_value }},
    y: {{ attributes.y|attribute_value }},
    width: {{ attributes.width|attribute_value }},
    height: {{ attributes.height|attribute_value }},
    view_box: {{ attributes.view_box|attribute_value }},
    stroke_linecap: {{ attributes.stroke_linecap|attribute_value }},
    stroke_linejoin: {{ attributes.stroke_linejoin|attribute_value }},
    stroke_width: {{ attributes.stroke_width|attribute_value }},
    stroke: {{ attributes.stroke|attribute_value }},
    fill: {{ attributes.fill|attribute_value }},
    data: r#"{{ icon.svg.content }}"#
};
{%- endfor %}

impl From<{{short_name|capitalize}}Icon> for icondata_core::IconData {
    fn from(icon: {{short_name|capitalize}}Icon) -> icondata_core::IconData {
        match icon {
            {% for icon in icons -%}
            {% let feat = icon.feature.name() %}
            #[cfg(feature = "{{feat}}")]
            LuIcon::{{feat}} => {{feat|shouty_snake_case}},
            {%- endfor %}
        }
    }
}
