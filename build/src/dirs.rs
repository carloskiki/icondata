use crate::{
    fs::{cargo_toml::CargoToml, readme_md::Readme, src_dir::SrcDir},
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
    src_dir: Option<SrcDir>,
    readme: Option<Readme>,
    ty: LibType,
}

impl Library {
    pub fn generate(&self, icons: &[SvgIcon]) -> Result<()> {
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
