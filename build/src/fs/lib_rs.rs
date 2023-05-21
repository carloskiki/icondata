use std::{io, path::PathBuf};

use anyhow::Result;
use askama::Template;
use heck::ToUpperCamelCase;
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

use crate::{
    dirs::{boilerplate::Boilerplate, icon_index::IconIndex, icon_library::IconLibrary, LibType},
    icon::SvgIcon,
    package::Package,
    Packages,
};

#[derive(Debug)]
pub(crate) struct LibRs {
    pub path: PathBuf,
}

#[derive(Template)]
#[template(path = "main_lib/lib.rs")]
struct IconIndexTemplate<'a> {
    short_names: Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "icon_lib/lib.rs")]
struct IconLibTemplate<'a> {
    icons: &'a [SvgIcon],
}

#[derive(Template)]
#[template(path = "main_lib/lib.rs")]
struct MainLibTemplate<'a> {
    lib_names: Vec<&'a str>,
}

impl LibRs {
    pub fn contents(lib_type: &LibType) -> Result<String> {
        match lib_type {
            LibType::IconLib(pkg) => {
                let icons = &Packages::get()?.icons[pkg.icon_range()];

                Ok(Template::render(&IconLibTemplate { icons })?)
            }
            LibType::MainLib => {
                let lib_names = Packages::get()?
                    .packages
                    .iter()
                    .map(|package| package.meta.short_name.as_ref())
                    .collect::<Vec<_>>();

                Ok(Template::render(&MainLibTemplate { lib_names })?)
            }
            LibType::IconIndex => {
                let short_names = Packages::get()?
                    .packages
                    .iter()
                    .map(|package| package.meta.short_name.as_ref())
                    .collect::<Vec<_>>();

                Ok(Template::render(&IconIndexTemplate { short_names })?)
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
