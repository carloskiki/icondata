# Icondata
This crate provides SVG icon data from popular and free icon libraries.

A [site](https://carloskiki.github.io/icondata) referencing every icon is available!

## Table of Contents
- [Icondata](#icondata)
- [Table of Contents](#table-of-contents)
- [Usage](#usage)
- [Icon Packages](#icon-packages)
- [Developing](#developing)
- [Contributing](#contributing)

## Usage
Every crate is tied to a specific icon package. You can find the name of the crate corresponding to a package in the [Icon Packages](#icon-packages) section.

Please see the [API documentation](https://docs.rs/icondata/latest/icondata/) to get started.

## Icon Packages
Licenses of the icons provided through these libraries were extracted with best intent,
but must only be taken as a hint. Please check the individual icon repositories for up-to-date license information.

| Icon Library | Version | Source | License | Crate Name |
| ------------ | ------- | ------ | ------- | ---------- |
| Ant Design Icons | 4.4.2 | Git: <https://github.com/ant-design/ant-design-icons> - Branch: master - Commit: f8416b8b99b91cbdcebb5b4450a815f2b8edc16f | MIT,  | icondata_ai |
| BoxIcons | 2.1.4 | Git: <https://github.com/atisawd/boxicons> - Branch: master - Commit: 9ffa9136e8681886bb7bd2145cd4098717ce1c11 | MIT,  | icondata_bi |
| Bootstrap Icons | 1.13.1 | Git: <https://github.com/twbs/icons> - Tag: v1.13.1 | MIT,  | icondata_bs |
| css.gg | 2.1.1 | Git: <https://github.com/astrit/css.gg> - Tag: 2.1.1 | MIT,  | icondata_cg |
| Charm | 0.18.0 | Git: <https://github.com/jaynewey/charm-icons> - Tag: v0.18.0 | MIT,  | icondata_ch |
| Font Awesome | 6.7.2 | Git: <https://github.com/FortAwesome/Font-Awesome> - Tag: 6.7.2 | CC BY 4.0,  | icondata_fa |
| Feather | 4.29.2 | Git: <https://github.com/feathericons/feather> - Tag: v4.29.2 | MIT,  | icondata_fi |
| Heroicons | 2.2.0 | Git: <https://github.com/refactoringui/heroicons> - Tag: v2.2.0 | MIT,  | icondata_hi |
| IcoMoon Free | unknown | Git: <https://github.com/Keyamoon/IcoMoon-Free> - Branch: master - Commit: d006795ede82361e1bac1ee76f215cf1dc51e4ca | CC BY 4.0, GPL,  | icondata_im |
| Ionicons | 8.0.9 | Git: <https://github.com/ionic-team/ionicons> - Tag: v8.0.9 | MIT,  | icondata_io |
| Lucide | 0.513.0 | Git: <https://github.com/lucide-icons/lucide> - Tag: 0.513.0 | ISC,  | icondata_lu |
| Material Design Icons | 7.4.47 | Git: <https://github.com/Templarian/MaterialDesign-SVG> - Tag: v7.4.47 | Apache 2.0,  | icondata_mdi |
| Github Octicons | 19.15.0 | Git: <https://github.com/primer/octicons> - Tag: v19.15.0 | MIT,  | icondata_oc |
| Remix Icon | 4.6.0 | Git: <https://github.com/Remix-Design/RemixIcon> - Tag: v4.6.0 | Apache 2.0,  | icondata_ri |
| Simple Icons | 15.0.0 | Git: <https://github.com/simple-icons/simple-icons> - Tag: 15.0.0 | CC0 1.0 Universal,  | icondata_si |
| Tabler Icons | 3.34.0 | Git: <https://github.com/tabler/tabler-icons> - Tag: v3.34.0 | MIT,  | icondata_tb |
| Typicons | 2.1.2 | Git: <https://github.com/stephenhutchings/typicons.font> - Tag: v2.1.2 | CC BY-SA 3.0,  | icondata_ti |
| VS Code Icons | 0.0.36 | Git: <https://github.com/microsoft/vscode-codicons> - Tag: 0.0.36 | CC BY 4.0,  | icondata_vs |
| Weather Icons | 2.0.12 | Git: <https://github.com/erikflowers/weather-icons> - Tag: 2.0.12 | SIL OFL 1.1,  | icondata_wi |

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