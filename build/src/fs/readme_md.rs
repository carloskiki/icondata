use anyhow::Result;
use indoc::{formatdoc, indoc};
use std::path::PathBuf;
use tokio::{io::AsyncWriteExt, fs::File};
use tracing::{error, instrument, trace};
use askama::Template;

use crate::{
    dirs::{base_repo::BaseRepo, icon_library::IconLibrary, LibType},
    package::{GitTarget, Package, PackageMetadata, PackageSource}, PACKAGES,
};

#[derive(Debug)]
pub struct Readme {
    pub path: PathBuf,
}

impl Readme {
    pub fn contents(ty: &LibType) ->  Result<String> {
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
                }.render()?)
            }

            LibType::MainLib => {
                #[derive(Template)]
                #[template(path = "main_lib/README.md")]
                struct ReadmeTemplate<'a> {
                    packages: Vec<(&'a str, String, String, String, &'a str)>,
                }

                let packages = Packages::get()?.packages.iter().map(|pkg| {
                    let version = match &pkg.meta.source {
                        PackageSource::Git { url: _, target } => match &target {
                            GitTarget::Branch {
                                ..,
                                version_hint,
                            } => version_hint.as_ref()
                            .map(|version| version.to_string())
                                .unwrap_or("unknown".to_owned()),
                                GitTarget::Tag { .., version } => {
                                    version.to_string()
                                }
                        },
                    };
                    let source = match &pkg.meta.source {
                        PackageSource::Git { url, target } => match &target {
                            GitTarget::Branch {
                                name,
                                commit_ref,
                                ..
                            } => format!("Git: <{url}> - Branch: {name} - Commit: {commit_ref}"),
                            GitTarget::Tag { name, version: _ } => {
                                format!("Git: <{url}> - Tag: {name}")
                            }
                        },
                    };
                    let license = pkg
                        .meta
                        .licenses
                        .iter()
                        .fold(String::new(), |mut acc, s| {
                            acc.push_str(s);
                            acc.push_str(", ");
                            acc
                        });

                    (
                        pkg.meta.short_name.as_ref(),
                        version,
                        source,
                        license,
                        pkg.meta.package_name.as_ref(),
                    )
                }).collect();

                Ok(ReadmeTemplate { packages }.render()?)
            }

            LibType::IconIndex => unreachable!("IconIndex does not have a README.md file"),

            LibType::Boilerplate => unreachable!("Boilerplate does not have a README.md file"),
        }
    }
}


