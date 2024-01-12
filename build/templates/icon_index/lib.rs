use icondata::*;

pub static ICONS: &[(&str, &str, icondata::Icon)] = &[
    {%- for icon in icons %}
    ("{{ icon }}", "{{ icon|lowercase }}", {{ icon }}),
{%- endfor ~%}
];
