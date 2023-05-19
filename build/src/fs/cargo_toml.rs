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
pub(crate) struct CargoToml {
    /// Path to the library's Cargo.toml file.
    pub path: PathBuf,
}


impl CargoToml {
    #[instrument(level = "info")]
    async fn create_file(&mut self) -> Result<tokio::fs::File> {
        trace!("Creating file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|err| {
                error!(?err, "Could not create file.");
                err
            })
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    async fn reset(&mut self) -> Result<tokio::fs::File> {
        if self.path.exists() {
            trace!("Removing file.");
            tokio::fs::remove_file(&self.path).await?;
        }

        trace!("Creating file.");
        self.create_file().await
    }

    #[instrument(level = "info", skip_all)]
    pub(crate) async fn generate(
        &mut self,
        lib_type: &LibType,
    ) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        let content = self.get_content(lib_type);

        let file = self.reset().await?;

        let writer = tokio::io::BufWriter::new(file);

        file.write_all(content.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;
        Ok(())
    }

    #[instrument(level = "info", skip_all)]
    fn get_content(lib_type: &LibType) -> Result<String> {
        info!(directory = ?lib_type, "Generating content for Cargo.toml.");
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

                let crate_name = format!("icondata_{}", pkg.meta.short_name);

                let icons = crate::Packages::get()?;
                let pkg_icons = icons[pkg.icon_range()];

                let features = pkg_icons.iter().map(|icon| {
                    &icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    crate_version: pkg.meta.crate_version.to_string(),
                    features,
                    icon_short_name: &pkg.meta.short_name,
                    icon_package_name: &pkg.meta.package_name,
                }.render()
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/Cargo.toml")]
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?.icons.iter().map(|icon| {
                    &icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    features,
                }.render()

            },

            LibType::IconIndex => {
                #[derive(Template)]
                #[template(path = "icon_index/Cargo.toml")]
                struct CargoTemplate<'a> {
                    features: Vec<&'a str>,
                }

                let features = crate::Packages::get()?.icons.iter().map(|icon| {
                    &icon.feature.name
                }).collect::<Vec<_>>();

                CargoTemplate {
                    features,
                }.render()
            },

            LibType::Boilerplate => todo!(),
        }
    }
}
