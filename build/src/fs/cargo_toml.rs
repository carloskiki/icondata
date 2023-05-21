use anyhow::Result;
use heck::ToUpperCamelCase;
use indoc::{formatdoc, indoc};
use std::{path::PathBuf, sync::Arc};
use tokio::io::AsyncWriteExt;
use tracing::{error, instrument, trace, info};
use askama::{Template, Template};

use crate::{
    dirs::{boilerplate::Boilerplate, icon_library::IconLibrary, LibType},
    icon::SvgIcon,
    package::{Package, PackageMetadata}, ICONS,
};

#[derive(Debug)]
pub struct CargoToml {
    /// Path to the library's Cargo.toml file.
    pub path: PathBuf,
}

impl CargoToml {
    pub fn contents(lib_type: &LibType) -> impl Template {
        match lib_type {
            LibType::IconLib(pkg) => {

                #[derive(Template)]
                #[template(path = "icon_lib/Cargo.toml")]
                struct CargoTemplate<'a> {
                    icon_short_name: &'a str,
                    crate_version: String,
                    icon_package_name: &'a str,
                    features: Vec<&'a str>,
                }

                let icons = crate::Packages::get()?.icons;
                let pkg_icons = &icons[pkg.icon_range()];

                let features = pkg_icons.iter().map(|icon| {
                    &*icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    crate_version: pkg.meta.crate_version.to_string(),
                    features,
                    icon_short_name: &pkg.meta.short_name,
                    icon_package_name: &pkg.meta.package_name,
                }
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/Cargo.toml")]
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?.icons.iter().map(|icon| {
                    &*icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    features,
                }

            },

            LibType::IconIndex => {
                #[derive(Template)]
                #[template(path = "icon_index/Cargo.toml")]
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?.icons.iter().map(|icon| {
                    &*icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    features,
                }
            },

            LibType::Boilerplate => todo!(),
        }
    }
}
