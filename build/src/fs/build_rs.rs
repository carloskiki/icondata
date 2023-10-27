use std::path::PathBuf;

use anyhow::Result;
use askama::Template;

use crate::dirs::LibType;


#[derive(Debug)]
pub struct BuildRs {
    pub path: PathBuf,
}

impl BuildRs {
    pub fn contents(lib_type: &LibType) -> Result<String> {
        match lib_type {
            LibType::IconLib(lib) => {
                #[derive(Template)]
                #[template(path = "icon_lib/build.rs", escape = "none")]
                struct Template<'a> {
                    short_name: &'a str,
                }

                Ok(Template {
                    short_name: &lib.meta.short_name,
                }.render()?)
            },
            LibType::MainLib => todo!(),
            _ => unimplemented!("icon index doesn't have a build.rs"),
        }
    }
}
