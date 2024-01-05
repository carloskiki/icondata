use std::path::PathBuf;

use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::{error, instrument, trace};

use crate::{
    fs::{build_rs::BuildRs, cargo_toml::CargoToml, lib_rs::LibRs, readme_md::Readme, icon_list::IconList},
    package::{Downloaded, Package},
};

#[derive(Debug)]
pub struct Library<'a> {
    cargo_toml: Option<CargoToml>,
    lib_rs: Option<LibRs>,
    readme: Option<Readme>,
    build_rs: Option<BuildRs>,
    icon_list: Option<IconList>,
    ty: LibType<'a>,
}

impl<'a> Library<'a> {
    pub fn new(mut path: PathBuf, ty: LibType<'a>) -> Self {
        let cargo_path = path.join("Cargo.toml");
        let lib_rs_path = path.join("src/lib.rs");
        let readme_path = path.join("README.md");
        match &ty {
            LibType::IconLib(_) => Library {
                cargo_toml: Some(CargoToml { path: cargo_path }),
                lib_rs: Some(LibRs { path: lib_rs_path }),
                readme: Some(Readme { path: readme_path }),
                build_rs: Some(BuildRs {
                    path: path.join("build.rs"),
                }),
                icon_list: Some(IconList {
                    path: path.join("ICON-LIST.txt"),
                }),
                ty,
            },

            LibType::MainLib => {
                path.pop();
                let readme_path = path.join("README.md");

                Library {
                    cargo_toml: Some(CargoToml { path: cargo_path }),
                    lib_rs: Some(LibRs { path: lib_rs_path }),
                    readme: Some(Readme { path: readme_path }),
                    icon_list: None,
                    build_rs: None,
                    ty,
                }
            }

            LibType::IconIndex => Library {
                cargo_toml: None,
                lib_rs: Some(LibRs { path: lib_rs_path }),
                ty,
                readme: None,
                icon_list: None,
                build_rs: None,
            },
        }
    }

    pub async fn generate(&self) -> Result<()> {
        if let Some(cargo_toml) = &self.cargo_toml {
            let contents = CargoToml::contents(&self.ty)?;
            write_to_file(&cargo_toml.path, contents).await?;
        };
        if let Some(lib_rs) = &self.lib_rs {
            let contents = LibRs::contents(&self.ty)?;
            write_to_file(&lib_rs.path, contents).await?;
        };
        if let Some(readme) = &self.readme {
            let contents = Readme::contents(&self.ty)?;
            write_to_file(&readme.path, contents).await?;
        };
        if let Some(build_rs) = &self.build_rs {
            let contents = BuildRs::contents(&self.ty)?;
            write_to_file(&build_rs.path, contents).await?;
        };
        if let Some(icon_list) = &self.icon_list {
            let contents = IconList::contents(&self.ty)?;
            write_to_file(&icon_list.path, contents).await?;
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum LibType<'a> {
    IconLib(&'a Package<Downloaded>),
    MainLib,
    IconIndex,
}

#[instrument(level = "info", skip(contents))]
pub async fn write_to_file(path: &PathBuf, contents: String) -> Result<()> {
    trace!(?path, "Making sure full path exists.");
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;

    if path.exists() {
        trace!("Removing existing file.");
        tokio::fs::remove_file(&path).await?;
    }

    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .await?;

    trace!(?path, "Writing contents to file.");
    file.write_all(contents.as_bytes()).await?;
    file.flush().await.map_err(|err| {
        error!(?err, "Could not flush file.");
        err
    })?;
    Ok(())
}
