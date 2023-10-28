//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!

/// This enum provides every icon as a variant.
/// It implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub enum {{short_name|capitalize}}Icon {
    {%- for (name, _) in name_svg %}
    #[cfg(any({{ name }}, icondata_include_all))]
    {{ name }},
    {%- endfor %}
}

{% for (name, svg) in name_svg.iter() -%}
#[cfg(any({{ name }}, icondata_include_all))]
const {{ name|shouty_snake_case }}: icondata_core::IconData = icondata_core::IconData {
    {% let attributes = svg.svg_attributes() -%}
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
    data: r###"{{ svg.content.as_str() }}"###
};
{% endfor %}
impl From<{{short_name|capitalize}}Icon> for icondata_core::IconData {
    fn from(icon: {{short_name|capitalize}}Icon) -> icondata_core::IconData {
        match icon {
            {%- for (name, _) in name_svg %}
            #[cfg(any({{ name }}, icondata_include_all))]
            {{short_name|capitalize}}Icon::{{name}} => {{name|shouty_snake_case}},
            {%- endfor %}
        }
    }
}
