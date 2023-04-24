use crate::fs::cargo_toml::CargoToml;
use anyhow::Result;
use indoc::{indoc, formatdoc};
use tokio::io::AsyncWriteExt;
use tracing::error;

use std::path::Path;

use super::icon_library::IconLibrary;

#[derive(Debug)]
pub(crate) struct IconIndex {
    cargo_toml: CargoToml<IconIndex>,
}

impl IconIndex {
    pub(crate) fn new(path: impl AsRef<Path>) -> Self {
        IconIndex {
            cargo_toml: CargoToml {
                path: path.as_ref().join("Cargo.toml"),
                _phantom: std::marker::PhantomData,
            },
        }
    }

    pub(crate) async fn generate(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        tracing::trace!("Generating icon-index.");
        self.cargo_toml.reset().await?;

        self.cargo_toml.write_cargo_toml(icon_libs).await
    }
}

impl CargoToml<IconIndex> {
    pub(crate) async fn write_cargo_toml(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        self.write_package_section().await?;
        self.write_dependencies(icon_libs).await
    }

    async fn write_package_section(&self) -> Result<()> {
        let package_section = indoc! {r#"
            [package]
            name = "icon_index"
            version = "0.1.0"
            edition = "2021"

            "#};

        let mut file = self.append().await?;

        file.write_all(package_section.as_bytes()).await?;
        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;
        Ok(())
    }

    async fn write_dependencies(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        let base_dependencies = indoc! {r#"
            [dependencies]
            leptos = { version = "0.2", default-features = false, features = ["csr", "serde", "stable"] }
            leptos_meta = { version = "0.2", default-features = false, features = ["csr"] }
            leptos_router = { version = "0.2", default-features = false, features = [
                "csr",
            ] }
            console_error_panic_hook = "0.1"
            console_log = "1"
            log = "0.4"
            icondata_core = { version = "0.0.1" }
            enum-iterator = "1"

            "#};

        let icon_features = icon_libs.iter().map(|lib| {
            lib.icons.iter().map(|icon| {
                format!("\"{}\",\n", icon.feature.name)
            }).collect::<String>()
        }).collect::<String>();

        let mut file = self.append().await?;

        file.write_all(base_dependencies.as_bytes()).await?;
        file.write_all("leptos_icons = { version = \"0.0.4\", default_features = false, features = [\n\"csr\",\n".as_bytes()).await?;

        file.write_all(icon_features.as_bytes()).await?;

        file.write_all("]}\n".as_bytes()).await?;

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;
        Ok(())

    }
}
