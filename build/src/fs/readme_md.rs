use anyhow::Result;
use askama::Template;
use std::path::PathBuf;

use crate::{
    dirs::LibType,
    package::{GitTarget, PackageSource},
    Packages,
};

#[derive(Debug)]
pub struct Readme {
    pub path: PathBuf,
}

impl Readme {
    pub fn contents(ty: &LibType) -> Result<String> {
        match ty {
            LibType::IconLib(pkg) => {
                #[derive(Template)]
                #[template(path = "icon_lib/README.md")]
                struct ReadmeTemplate<'a> {
                    short_name: &'a str,
                    package_name: &'a str,
                }

                Ok(ReadmeTemplate {
                    short_name: &pkg.meta.short_name,
                    package_name: &pkg.meta.package_name,
                }
                .render()?)
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/README.md")]
                struct ReadmeTemplate<'a> {
                    packages: Vec<(&'a str, String, String, String, &'a str)>,
                }

                let packages = Packages::get()?
                    .iter()
                    .map(|pkg| {
                        let version = match &pkg.meta.source {
                            PackageSource::Git { target, .. } => match &target {
                                GitTarget::Branch { version_hint, .. } => version_hint
                                    .as_ref()
                                    .map(|version| version.to_string())
                                    .unwrap_or("unknown".to_owned()),
                                GitTarget::Tag { version, .. } => version.to_string(),
                            },
                        };
                        let source = match &pkg.meta.source {
                            PackageSource::Git { url, target } => match &target {
                                GitTarget::Branch {
                                    name, commit_ref, ..
                                } => {
                                    format!("Git: <{url}> - Branch: {name} - Commit: {commit_ref}")
                                }
                                GitTarget::Tag { name, version: _ } => {
                                    format!("Git: <{url}> - Tag: {name}")
                                }
                            },
                        };
                        let license = pkg.meta.licenses.iter().fold(String::new(), |mut acc, s| {
                            acc.push_str(s);
                            acc.push_str(", ");
                            acc
                        });

                        (
                            pkg.meta.package_name.as_ref(),
                            version,
                            source,
                            license,
                            pkg.meta.short_name.as_ref(),
                        )
                    })
                    .collect();

                Ok(ReadmeTemplate { packages }.render()?)
            }
            LibType::IconIndex => unreachable!("IconIndex does not have a README.md file"),
        }
    }
}
