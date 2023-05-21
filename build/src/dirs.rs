use std::path::PathBuf;

use tracing::instrument;
use anyhow::Result;

use crate::{
    fs::{cargo_toml::CargoToml, readme_md::Readme, src_dir::SrcDir, lib_rs::{LibRs, self}},
    package::{Downloaded, Package}, icon::SvgIcon,
};

#[derive(Debug)]
pub struct Library {
    path: PathBuf,
    cargo_toml: Option<CargoToml>,
    lib_rs: Option<LibRs>,
    readme: Option<Readme>,
    ty: LibType,
}

impl Library {
    pub fn new(mut path: PathBuf, ty: LibType) -> Self {
        match &ty {
            LibType::IconLib(_) => {
                let cargo_toml_path = path.join("Cargo.toml");
                let lib_rs_path = src_dir_path.join("src/lib.rs");
                let readme_path = path.join("README.md");

                Library {
                    path,
                    cargo_toml: Some(CargoToml { path: cargo_toml_path }),
                    lib_rs: Some(LibRs { path: lib_rs_path }),
                    readme: Some(Readme {
                        path: readme_path,
                    }),
                    ty,
                }
            },

            LibType::MainLib => {
                let cargo_toml_path = path.join("Cargo.toml");
                let lib_rs_path = src_dir_path.join("src/lib.rs");
                let lib_path = path.clone();

                path.pop();
                let readme_path = path.join("README.md");

                Library {
                    path: lib_path,
                    cargo_toml: Some(CargoToml { path: cargo_toml_path }),
                    lib_rs: Some(LibRs { path: lib_rs_path }),
                    readme: Some(Readme {
                        path: readme_path,
                    }),
                    ty,
                }
            },

            LibType::IconIndex => {
                let cargo_toml_path = path.join("Cargo.toml");
                let lib_rs_path = src_dir_path.join("src/lib.rs");

                Library {
                    path,
                    cargo_toml: Some(CargoToml { path: cargo_toml_path }),
                    lib_rs: Some(LibRs { path: lib_rs_path }),
                    readme: None,
                    ty,
                }
            },

            LibType::Boilerplate => {
                Library {
                    path,
                    cargo_toml: Some(CargoToml { path: path.join("Cargo.toml") }),
                    lib_rs: None,
                    readme: None,
                    ty,
                }
            }
        }
    }

    pub async fn generate(&self) -> Result<()> {
        if let Some(cargo_toml) = self.cargo_toml {
            let contents = CargoToml::template(&self.ty);
            write_to_file(&cargo_toml.path, contents).await?;
        };
        if let Some(lib_rs) = self.lib_rs {
            let contents = LibRs::template(&self.ty);
            write_to_file(&src_dir.path, contents).await?;
        };
        if let Some(readme) = self.readme {
            let contents = Readme::template(&self.ty);
            write_to_file(&readme.path, contents).await?;
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum LibType {
    IconLib(Package<Downloaded>),
    MainLib,
    IconIndex,
    Boilerplate,
}

#[instrument(level = "info", skip(contents))]
pub async fn write_to_file(path: &PathBuf, contents: String) -> Result<()> {
    trace!(?path, "Making sure full path exists.");
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;

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
    })
}
