use std::{io, path::PathBuf};

use anyhow::Result;
use heck::ToUpperCamelCase;
use indoc::indoc;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use snafu::{prelude::*, Backtrace, OptionExt};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};
use askama::{Template, Template};

use crate::{
    dirs::{boilerplate::Boilerplate, icon_index::IconIndex, icon_library::IconLibrary, LibType},
    icon::SvgIcon,
    package::Package, Packages,
};

#[derive(Debug)]
pub(crate) struct LibRs {
    pub path: PathBuf,
}

impl LibRs {
    pub fn template(lib_type: &LibType) -> impl Template {
        match lib_type {
            LibType::IconLib(pkg) => {
                #[derive(Template)]
                #[template(path = "icon_lib/lib.rs")]
                struct LibTemplate<'a> {
                    icons: &'a [SvgIcon],
                }

                let icons = &Packages::get()?.icons[pkg.icon_range()];

                LibTemplate { icons }
            },
            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/lib.rs")]
                struct LibTemplate<'a> {
                    lib_names: Vec<&'a str>,
                }

                let lib_names = Packages::get()?.packages.iter().map(|package| {
                    package.meta.short_name.as_ref()
                }).collect::<Vec<_>>();

                LibTemplate {
                    lib_names,
                }
            },
            LibType::IconIndex => {

                #[derive(Template)]
                #[template(path = "main_lib/lib.rs")]
                struct LibTemplate {
                    short_names_cap: Vec<String>,
                }

                let short_names_cap = Packages::get()?.packages.iter().map(|package| {
                    package.meta.short_name.to_upper_camel_case()
                }).collect::<Vec<_>>();

                LibTemplate {
                    short_names_cap,
                }
            }

            LibType::Boilerplate => unreachable!("Boilerplate does not have a lib.rs file"),
        }
    }
}

mod filters {
    use heck::ToShoutySnakeCase;
    pub fn shouty_snake_case<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let input = s.to_string();
        Ok(input.to_shouty_snake_case())
    }
}
