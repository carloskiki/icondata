use std::path::PathBuf;

use anyhow::Result;

use crate::dirs::LibType;


#[derive(Debug)]
pub struct IconList {
    pub path: PathBuf,
}

impl IconList {
    pub fn contents(lib_type: &LibType) -> Result<String> {
        match lib_type {
            LibType::IconLib(lib) => {
                Ok(lib.icons().iter().map(|icon| format!("{},", icon.name)).collect())
            },
            _ => unimplemented!("icon index and main lib don't have a build.rs"),
        }
    }
}
