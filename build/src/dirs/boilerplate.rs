use std::path::PathBuf;

use anyhow::Result;
use tracing::{info, instrument, trace};

use crate::fs::{cargo_toml::CargoToml, lib_rs::LibRs, src_dir::SrcDir};

use super::icon_library::IconLibrary;

#[derive(Debug)]
pub(crate) struct Boilerplate {
    pub path: PathBuf,
    pub cargo_toml: CargoToml<Boilerplate>,
    pub src_dir: SrcDir<Boilerplate>,
}

impl Boilerplate {
    pub fn new(root: PathBuf) -> Self {
        Self {
            path: root.clone(),
            cargo_toml: CargoToml {
                path: root.join("Cargo.toml"),
            },
            src_dir: SrcDir {
                path: root.join("src"),
                lib_rs: LibRs {
                    path: root.join("src").join("lib.rs"),
                },
            },
        }
    }

    #[instrument(level = "info", skip_all)]
    pub async fn generate(&mut self, icon_libs: &[IconLibrary]) -> Result<()> {
        trace!("Ensuring library directory exists.");
        if !self.path.exists() {
            tokio::fs::create_dir_all(&self.path).await?;
        }

        trace!("Resetting library directory.");
        self.src_dir.reset().await?;
        self.cargo_toml.reset().await?;
        self.cargo_toml.write_cargo_toml(&icon_libs).await?;

        self.src_dir.lib_rs.write_lib_rs().await?;

        info!("Library generated.");
        Ok(())
    }
}
