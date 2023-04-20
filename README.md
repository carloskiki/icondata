# Leptos-Icons

This is the parent for:

| Crate             | Path               | Type | Description                                      |
| ---               | ---                | ---  | ---                                              |
| builder           | /build             | bin  | Generates the `icondata` library and all of the sub libraries.            |
| icondata      | /icondata      | lib  | Actual library published on crates.io.           |
| icondata_core | /icondata_core | lib  | Helpers and utility functions                    |
| icondata_*   | /icondata_*   | lib  | A library containing all icons from package "*" |

## Executing commands

This repository uses [Just](https://github.com/casey/just)

Simply call

    just

to see a list of available commands.

You may need to install `just` using

    cargo install just
