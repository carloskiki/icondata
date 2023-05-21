use anyhow::Result;
use askama::Template;
use std::path::PathBuf;

use crate::dirs::LibType;

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
                struct CargoTemplate<'a> {
                    short_name: &'a str,
                    crate_version: String,
                    icon_package_name: &'a str,
                    features: Vec<&'a str>,
                }

                let icons = &crate::Packages::get()?.icons;
                let pkg_icons = &icons[pkg.icon_range().clone()];

                let features = pkg_icons
                    .iter()
                    .map(|icon| &*icon.feature.name)
                    .collect::<Vec<_>>();

                Ok(CargoTemplate {
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
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?
                    .icons
                    .iter()
                    .map(|icon| &*icon.feature.name)
                    .collect::<Vec<_>>();

                Ok(CargoTemplate { features }.render()?)
            }

            LibType::IconIndex => {
                #[derive(Template)]
                #[template(path = "icon_index/Cargo.toml", escape = "none")]
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?
                    .icons
                    .iter()
                    .map(|icon| &*icon.feature.name)
                    .collect::<Vec<_>>();

                Ok(CargoTemplate { features }.render()?)
            }

            LibType::Boilerplate => Ok("test".to_string()),
        }
    }
}
