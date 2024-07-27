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
                struct Template<'a> {
                    short_name: &'a str,
                    crate_version: String,
                    icon_package_name: &'a str,
                }

                Ok(Template {
                    crate_version: pkg.meta.crate_version.to_string(),
                    short_name: &pkg.meta.short_name,
                    icon_package_name: &pkg.meta.package_name,
                }
                .render()?)
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/Cargo.toml", escape = "none")]
                struct Template<'a> {
                    sn_version: Vec<(&'a str, String)>,
                }

                let sn_version: Vec<_> = crate::Packages::get()?
                    .iter()
                    .map(|package| {
                        (
                            &*package.meta.short_name,
                            package.meta.crate_version.to_string(),
                        )
                    })
                    .collect();

                Ok(Template { sn_version }.render()?)
            }

            LibType::IconIndex => unimplemented!("IconIndex does not generate a Cargo.toml file."),
        }
    }
}
