use anyhow::Result;
use askama::Template;
use std::path::PathBuf;

use crate::{dirs::LibType, Packages};

#[derive(Debug)]
pub struct CargoToml {
    /// Path to the library's Cargo.toml file.
    pub path: PathBuf,
}

impl CargoToml {
    pub fn contents(lib_type: &LibType) -> Result<String> {
        match lib_type {
            LibType::IconLib(pkg) => {
                #[derive(Template)]
                #[template(path = "icon_lib/Cargo.toml", escape = "none")]
                struct Template<'a> {
                    short_name: &'a str,
                    crate_version: String,
                    icon_package_name: &'a str,
                    features: Vec<&'a str>,
                }

                let pkg_icons = pkg.icons();
                let features = pkg_icons
                    .iter()
                    .map(|icon| &*icon.feature.name)
                    .collect::<Vec<_>>();

                Ok(Template {
                    crate_version: pkg.meta.crate_version.to_string(),
                    features,
                    short_name: &pkg.meta.short_name,
                    icon_package_name: &pkg.meta.package_name,
                }
                .render()?)
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/Cargo.toml", escape = "none")]
                struct Template<'a> {
                    sn_features: Vec<(&'a str, Vec<&'a str>)>,
                    sn_version: Vec<(&'a str, String)>,
                }

                let (sn_features, sn_version): (Vec<_>, Vec<_>) = crate::Packages::get()?.iter().map(|package| {
                    let features = package
                        .icons()
                        .iter()
                        .map(|icon| &*icon.feature.name)
                        .collect::<Vec<_>>();

                    ((&*package.meta.short_name, features), (&*package.meta.short_name, package.meta.crate_version.to_string()))
                }).unzip();

                Ok(Template { sn_features, sn_version }.render()?)
            }

            LibType::IconIndex => {
                #[derive(Template)]
                #[template(path = "icon_index/Cargo.toml", escape = "none")]
                struct Template<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get_icons()?
                    .map(|icon| &*icon.feature.name)
                    .collect::<Vec<_>>();

                Ok(Template { features }.render()?)
            }

            LibType::Boilerplate => {
                #[derive(Template)]
                #[template(path = "boilerplate/Cargo.toml", escape = "none")]
                struct Template<'a> {
                    features: Vec<&'a str>,
                    short_names: Vec<&'a str>,
                }

                let short_names = Packages::get()?
                    .iter()
                    .map(|pkg| pkg.meta.short_name.as_ref())
                    .collect::<Vec<_>>();
                let features = vec![];

                Ok(Template {
                    short_names,
                    features,
                }
                .render()?)
            }
        }
    }
}
