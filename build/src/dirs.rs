use crate::{
    fs::{cargo_toml::CargoToml, readme_md::Readme, src_dir::SrcDir, lib_rs::{LibRs, self}},
    package::{Downloaded, Package}, icon::SvgIcon,
};

pub mod base_repo;
pub mod boilerplate;
pub mod icon_index;
pub mod icon_library;
pub mod main_library;

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

    pub fn generate(&self) -> Result<()> {
        if let Some(cargo_toml) = self.cargo_toml {
            cargo_toml.generate(&self.ty).await?;
        };
        if let Some(src_dir) = self.src_dir {
            src_dir.generate(&self.ty).await?;
        };
        if let Some(readme) = self.readme {
            readme.generate(&self.ty).await?;
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
