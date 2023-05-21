# Icondata

This crate provides SVG icon data from popular and free icon libraries. Every icon is packaged as its own cargo feature to reduce build times.

Please note that as long as version 0.1.0 is not published, any patch update may cause breaking changes and is not guaranteed to work.

## Table of Contents

- [Icondata](#icondata)
- [Table of Contents](#table-of-contents)
- [Usage](#usage)
- [Icon Packages](#icon-packages)
- [Developing](#developing)
- [Contributing](#contributing)

## Usage

Every crate is tied to a specific icon package. You can find the name of the crate corresponding to a package in the [Icon Packages](#icon-packages) section.

As an example, to use the bootstrap folder icon, add the following to your `Cargo.toml`:

```toml
[dependencies]
# ...
icondata_bs = { version = "...", features = ["BsFolder"] }
```

If you are creating a component library for a web framework, you can use this [boilerplate](/boilerplate) setup.

## Icon Packages

Licenses of the icons provided through these libraries were extracted with best intent,
but must only be taken as a hint. Please check the individual icon repositories for up-to-date license information.

| Icon Library | Version | Source | License | Crate Name |
| ------------ | ------- | ------ | ------- | ---------- |
{% for (pack_name, version, source, license, short_name) in packages %}
| {{pack_name}} | {{version}} | {{source}} | {{license}} | icondata_{{short_name}} |
{% endfor %}

| Icon Library | Version | Source | License | Crate Name |
| ------------ | ------- | ------ | ------- | ---------- |
| Ant Design Icons | 5.3.2   | Git: <https://github.com/ant-design/ant-design-icons> - Branch: master - Commit: 7c804893b4ac698d5713b2b59f3d044eb8f5128f | MIT,                | icondata_ai |
| Font Awesome     | 6.4.0   | Git: <https://github.com/FortAwesome/Font-Awesome> - Tag: 6.4.0                                                           | CC BY 4.0,          | icondata_fa |
| Weather Icons    | 2.0.12  | Git: <https://github.com/erikflowers/weather-icons> - Tag: 2.0.12                                                         | SIL OFL 1.1,        | icondata_wi |
| Feather          | 4.29.0  | Git: <https://github.com/feathericons/feather> - Tag: v4.29.0                                                             | MIT,                | icondata_fi |
| VS Code Icons    | 0.0.32  | Git: <https://github.com/microsoft/vscode-codicons> - Tag: 0.0.32                                                         | CC BY 4.0,          | icondata_vs |
| Bootstrap Icons  | 1.10.4  | Git: <https://github.com/twbs/icons> - Tag: v1.10.4                                                                       | MIT,                | icondata_bs |
| BoxIcons         | 2.1.4   | Git: <https://github.com/atisawd/boxicons> - Branch: master - Commit: 9ffa9136e8681886bb7bd2145cd4098717ce1c11            | CC BY 4.0,          | icondata_bi |
| IcoMoon Free     | unknown | Git: <https://github.com/Keyamoon/IcoMoon-Free> - Branch: master - Commit: d006795ede82361e1bac1ee76f215cf1dc51e4ca       | CC BY 4.0, GPL,     | icondata_im |
| Ionicons         | 7.1.0   | Git: <https://github.com/ionic-team/ionicons> - Tag: v7.1.0                                                               | MIT,                | icondata_io |
| Remix Icon       | 3.2.0   | Git: <https://github.com/Remix-Design/RemixIcon> - Tag: v3.2.0                                                            | Apache 2.0,         | icondata_ri |
| Simple Icons     | 8.10.0  | Git: <https://github.com/simple-icons/simple-icons> - Tag: 8.10.0                                                         | CC0 1.0 Universal,  | icondata_si |
| Typicons         | 2.1.2   | Git: <https://github.com/stephenhutchings/typicons.font> - Tag: v2.1.2                                                    | CC BY-SA 3.0,       | icondata_ti |
| Heroicons        | 2.0.17  | Git: <https://github.com/refactoringui/heroicons> - Tag: v2.0.17                                                          | MIT,                | icondata_hi |
| css.gg           | 2.0.0   | Git: <https://github.com/astrit/css.gg> - Tag: 2.0.0                                                                      | MIT,                | icondata_cg |
| Tabler Icons     | 2.17.0  | Git: <https://github.com/tabler/tabler-icons> - Tag: v2.17.0                                                              | MIT,                | icondata_tb |
| Github Octicons  | 18.3.0  | Git: <https://github.com/primer/octicons> - Tag: v18.3.0                                                                  | MIT,                | icondata_oc |
| Lucide           | 0.172.0 | Git: <https://github.com/lucide-icons/lucide> - Tag: v0.172.0                                                             | ISC,                | icondata_lu |
| Charm            | 0.8.0   | Git: <https://github.com/jaynewey/charm-icons> - Tag: v0.18.0                                                             | MIT,                | icondata_ch |

## Developing

This repository uses Just

Simply call
```bash
just
```
to see a list of available commands.

You may need to install just using

```bash
cargo install just
```

## Contributing

Contributions are more than welcomed!
Do not hesitate add icon libraries, features, etc.
