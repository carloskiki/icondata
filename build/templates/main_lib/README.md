# Icondata

This crate provides SVG icon data from popular and free icon libraries. Every crate is packaged as its own cargo feature to reduce build times.

A [site](https://carlosted.github.io/icondata) referencing every icon is now available!

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
{% for (pack_name, version, source, license, short_name) in packages -%}
| {{pack_name}} | {{version}} | {{source}} | {{license}} | icondata_{{short_name}} |
{% endfor %}
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
