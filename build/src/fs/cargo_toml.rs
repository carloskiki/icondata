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

impl CargoToml<Boilerplate> {
    pub(crate) async fn write_cargo_toml(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        self.write_package_section().await?;
        self.write_dependencies_section().await?;
        self.write_features_section(icon_libs).await?;

        Ok(())
    }

    async fn write_package_section(&self) -> Result<()> {
        let package_section = formatdoc! {r#"
                [package]
                # ---------------
                # Package section
                # ---------------

                "#};

        let mut file = self.append().await?;

        file.write_all(package_section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;
        Ok(())
    }

    #[instrument(level = "info")]
    async fn write_dependencies_section(&self) -> Result<()> {
        let mut file = self.append().await?;
        let dependencies = indoc! {r#"
            [dependencies]
            icondata_core = "0.0.2"

        "#};
        file.write_all(dependencies.as_bytes()).await?;

        for lib in Package::all() {
            file
                // Example: icondata_ai = { path = "../icondata_ai" }
                .write_all(
                    format!(
                        "icondata_{short_name} = {{  version = \"{crate_version}\", optional = true }}\n",
                        short_name = &lib.meta.short_name,
                        crate_version = lib.meta.crate_version,
                    )
                    .as_bytes(),
                )
                .await?;
        }

        file.write_all("\n".as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }

    #[instrument(level = "info", skip(icon_libs))]
    async fn write_features_section(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        let mut writer = self.append().await?;

        fn icondata_feature_list(feature_name: &str) -> String {
            Package::all()
                .iter()
                .map(|pack| format!("\n\"icondata_{}/{feature_name}\",", pack.meta.short_name))
                .collect::<String>()
        }

        let base_features = formatdoc! {r#"
            [features]
            default = []
            serde = [{serde}
            ]
            strum = [{strum}
            ]

            "#, serde = icondata_feature_list("serde"),
            strum = icondata_feature_list("strum")
        };

        writer.write_all(base_features.as_bytes()).await?;

        for package in Package::all() {
            writer
                // Example: Ai = []
                .write_all(
                    format!(
                        "{lib_short_name} = []\n",
                        lib_short_name = &package.meta.short_name.to_upper_camel_case(),
                    )
                    .as_bytes(),
                )
                .await?;
        }

        for lib in icon_libs.iter() {
            for icon in &lib.icons {
                writer
                    // Example: AiPushpinTwotone = ["Ai", "icondata_ai/AiPushpinTwotone"]
                    .write_all(
                        format!(
                            "{feature_name} = [\"{camel_short_name}\", \"icondata_{short_name}/{feature_name}\"]\n",
                            camel_short_name = &lib.package.meta.short_name.to_upper_camel_case(),
                            short_name = &lib.package.meta.short_name,
                            feature_name = icon.feature.name,
                        )
                        .as_bytes(),
                    )
                    .await?;
            }
        }
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }
}
