use anyhow::Result;
use snafu::{prelude::*, Backtrace};
use std::{borrow::Cow, marker::PhantomData, path::PathBuf};
use strum::{EnumIter, IntoEnumIterator};
use tracing::{info, instrument};

use crate::{git, icon::SvgIcon, path, sem_ver::SemVer};

mod reader;

/// Name of the directory, relative to the root of this crate, to which all icon packages should be downloaded.
const DOWNLOAD_DIR: &str = "downloads";

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Could not perform 'git checkout' {path:?}"))]
    GitCheckout {
        source: git::Error,
        target: GitTarget,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Could not perform 'git clone' {path:?}"))]
    GitClone {
        source: git::Error,
        url: String,
        target: GitTarget,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Could not perform 'git clone --no-checkout' {path:?}"))]
    GitCloneWithoutCheckout {
        source: git::Error,
        url: String,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Could not read icon data from {path:?}"))]
    IconRead {
        source: anyhow::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
}

#[derive(Debug, Clone)]
pub struct Package<S> {
    pub ty: PackageType,
    pub meta: PackageMetadata,
    icons: Vec<SvgIcon>,
    phantom_data: PhantomData<S>,
}

/// It is not guaranteed that the package was downloaded to the exact version specified.
#[derive(Debug, Clone)]
pub struct Unknown {}

/// The package was successfully downloaded, and Icon data was read successfully.
#[derive(Debug, Clone)]
pub struct Downloaded {}

impl<S> Package<S> {
    pub fn download_path(&self) -> PathBuf {
        path::build_crate(DOWNLOAD_DIR).join(self.meta.download_dir.as_ref())
    }
}

impl Package<Unknown> {
    pub fn all() -> Vec<Package<Unknown>> {
        Package::<Unknown>::of_types(PackageType::iter())
    }

    fn of_types<I: Iterator<Item = PackageType>>(types: I) -> Vec<Package<Unknown>> {
        types
            .map(|ty| Package::<Unknown> {
                ty,
                meta: ty.metadata(),
                icons: Vec::new(),
                phantom_data: PhantomData {},
            })
            .collect()
    }

    #[instrument(level = "info", fields(package = self.meta.package_name.as_ref()))]
    pub async fn remove(&self) -> Result<()> {
        let download_path = self.download_path();
        if download_path.exists() {
            info!(?download_path, "Removing...");
            tokio::fs::remove_dir_all(&download_path).await?;
        }
        Ok(())
    }

    #[instrument(level = "info", skip_all)]
    pub(crate) async fn download(self) -> Result<Package<Downloaded>> {
        let download_path = self.download_path();
        info!(?download_path, "Downloading...");

        match &self.meta.source {
            PackageSource::Git { url, target } => {
                if download_path.exists() {
                    info!(?download_path, "Directory exists. Assuming git repository.");
                    git::checkout(target, &download_path).with_context(|_| GitCheckoutSnafu {
                        target: target.clone(),
                        path: download_path.clone(),
                    })?;
                } else {
                    info!(
                        ?download_path,
                        "Directory does not exist. Cloning the repository."
                    );
                    git::clone(url, target, &download_path)
                        .with_context(|_| GitCloneSnafu {
                            url: url.clone(),
                            target: target.clone(),
                            path: download_path.clone(),
                        })
                        .or_else(|_err| {
                            info!("Direct clone unsuccessful. Trying clone with checkout...");
                            git::clone_without_checkout(url, &download_path).with_context(
                                |_| GitCloneWithoutCheckoutSnafu {
                                    url: url.clone(),
                                    path: download_path.clone(),
                                },
                            )?;
                            git::checkout(target, &download_path).with_context(|_| {
                                GitCheckoutSnafu {
                                    target: target.clone(),
                                    path: download_path.clone(),
                                }
                            })
                        })?;
                }
            }
        };

        let icons_path = self.download_path().join(self.meta.svg_dir.as_ref());
        let mut icons = reader::read_icons(&self, icons_path.clone()).await?;
        icons.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Package::<Downloaded> {
            ty: self.ty,
            meta: self.meta,
            icons,
            phantom_data: PhantomData {},
        })
    }
}

impl Package<Downloaded> {
    pub fn icons(&self) -> &[SvgIcon] {
        &self.icons
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Clone, Copy)]
pub enum PackageType {
    AntDesignIcons,
    FontAwesome,
    WeatherIcons,
    Feather,
    VSCodeIcons,
    BootstrapIcons,
    BoxIcons,
    IcoMoonFree,
    Ionicons,
    RemixIcon,
    SimpleIcons,
    Typicons,
    Heroicons,
    CssGg,
    TablerIcons,
    GithubOcticons,
    Lucide,
    Charm,
    MaterialDesignIcons,
}

#[derive(Debug, Clone)]
pub struct PackageMetadata {
    /// Two-character identifier like "fa" for "Font Awesome".
    pub short_name: Cow<'static, str>,
    /// Full human readable name of this icon package.
    pub package_name: Cow<'static, str>,
    /// Licenses of the icon package.
    pub licenses: &'static [Cow<'static, str>],
    /// The source of this icon package.
    pub source: PackageSource,
    /// Directory to which the source should be downloaded.
    pub download_dir: Cow<'static, str>,
    /// Directory relative to download_dir under which raw SVG files can be found.
    pub svg_dir: Cow<'static, str>, // TODO: PathBuf?
    pub crate_version: SemVer,
}

#[derive(Debug, Clone)]
pub enum PackageSource {
    Git {
        url: Cow<'static, str>, // TODO use url type?
        target: GitTarget,
    },
}

#[derive(Debug, Clone)]
pub enum GitTarget {
    Branch {
        name: Cow<'static, str>,
        commit_ref: Cow<'static, str>,
        version_hint: Option<SemVer>,
    },
    Tag {
        name: Cow<'static, str>,
        version: SemVer,
    },
}

impl PackageType {
    /// Test whether a particular string represents a category of this icon package.
    pub fn is_category(&self, str: &str) -> bool {
        // We avoid using all-numeric directories as categories,
        // as they most likely state the size of the icons contained and not an actual category.
        if str.chars().all(char::is_numeric) {
            return false;
        }
        match self {
            // SVG's located in the "logos" directory are distinct from files in the "regular" and "solid" directories. We may not use that as a category.
            PackageType::BoxIcons => str != "logos",
            _ => true,
        }
    }

    fn metadata(&self) -> PackageMetadata {
        match self {
            PackageType::AntDesignIcons => PackageMetadata {
                short_name: Cow::Borrowed("ai"),
                package_name: Cow::Borrowed("Ant Design Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/ant-design/ant-design-icons"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("e09efdead14961d3cc9ec0c24a182f66241436de"),
                        version_hint: Some(SemVer {
                            major: 4,
                            minor: 3,
                            patch: 1,
                            prerelease: None,
                            build: None,
                        }),
                    },
                },
                download_dir: "ant-design-icons".into(),
                svg_dir: Cow::Borrowed("packages/icons-svg/svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::FontAwesome => PackageMetadata {
                short_name: Cow::Borrowed("fa"),
                package_name: Cow::Borrowed("Font Awesome"),
                licenses: &[Cow::Borrowed("CC BY 4.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/FortAwesome/Font-Awesome"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("6.4.2"),
                        version: SemVer {
                            major: 6,
                            minor: 4,
                            patch: 2,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("font-awesome"),
                svg_dir: Cow::Borrowed("svgs"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::WeatherIcons => PackageMetadata {
                short_name: Cow::Borrowed("wi"),
                package_name: Cow::Borrowed("Weather Icons"),
                licenses: &[Cow::Borrowed("SIL OFL 1.1")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/erikflowers/weather-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("2.0.12"),
                        version: SemVer {
                            major: 2,
                            minor: 0,
                            patch: 12,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("weather-icons"),
                svg_dir: Cow::Borrowed("svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Feather => PackageMetadata {
                short_name: Cow::Borrowed("fi"),
                package_name: Cow::Borrowed("Feather"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/feathericons/feather"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v4.29.1"),
                        version: SemVer {
                            major: 4,
                            minor: 29,
                            patch: 1,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("feather"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::VSCodeIcons => PackageMetadata {
                short_name: Cow::Borrowed("vs"),
                package_name: Cow::Borrowed("VS Code Icons"),
                licenses: &[Cow::Borrowed("CC BY 4.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/microsoft/vscode-codicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("0.0.33"),
                        version: SemVer {
                            major: 0,
                            minor: 0,
                            patch: 33,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("vscode-codicons"),
                svg_dir: Cow::Borrowed("src/icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::BootstrapIcons => PackageMetadata {
                short_name: Cow::Borrowed("bs"),
                package_name: Cow::Borrowed("Bootstrap Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/twbs/icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v1.11.0"),
                        version: SemVer {
                            major: 1,
                            minor: 11,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("bootstrap-icons"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::BoxIcons => PackageMetadata {
                short_name: Cow::Borrowed("bi"),
                package_name: Cow::Borrowed("BoxIcons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/atisawd/boxicons"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("9ffa9136e8681886bb7bd2145cd4098717ce1c11"),
                        version_hint: Some(SemVer {
                            major: 2,
                            minor: 1,
                            patch: 4,
                            prerelease: None,
                            build: None,
                        }),
                    },
                },
                download_dir: Cow::Borrowed("boxicons"),
                svg_dir: Cow::Borrowed("svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::IcoMoonFree => PackageMetadata {
                short_name: Cow::Borrowed("im"),
                package_name: Cow::Borrowed("IcoMoon Free"),
                licenses: &[Cow::Borrowed("CC BY 4.0"), Cow::Borrowed("GPL")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/Keyamoon/IcoMoon-Free"),
                    target: GitTarget::Branch {
                        name: Cow::Borrowed("master"),
                        commit_ref: Cow::Borrowed("d006795ede82361e1bac1ee76f215cf1dc51e4ca"),
                        version_hint: None,
                    },
                },
                download_dir: Cow::Borrowed("icomoon-free"),
                svg_dir: Cow::Borrowed("SVG"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Ionicons => PackageMetadata {
                short_name: Cow::Borrowed("io"),
                package_name: Cow::Borrowed("Ionicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/ionic-team/ionicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v7.1.2"),
                        version: SemVer {
                            major: 7,
                            minor: 1,
                            patch: 2,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("ionicons"),
                svg_dir: Cow::Borrowed("src/svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::RemixIcon => PackageMetadata {
                short_name: Cow::Borrowed("ri"),
                package_name: Cow::Borrowed("Remix Icon"),
                licenses: &[Cow::Borrowed("Apache 2.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/Remix-Design/RemixIcon"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v3.5.0"),
                        version: SemVer {
                            major: 3,
                            minor: 5,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("RemixIcon"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::SimpleIcons => PackageMetadata {
                short_name: Cow::Borrowed("si"),
                package_name: Cow::Borrowed("Simple Icons"),
                licenses: &[Cow::Borrowed("CC0 1.0 Universal")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/simple-icons/simple-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("9.14.0"),
                        version: SemVer {
                            major: 9,
                            minor: 14,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("simple-icons"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Typicons => PackageMetadata {
                short_name: Cow::Borrowed("ti"),
                package_name: Cow::Borrowed("Typicons"),
                licenses: &[Cow::Borrowed("CC BY-SA 3.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/stephenhutchings/typicons.font"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.1.2"),
                        version: SemVer {
                            major: 2,
                            minor: 1,
                            patch: 2,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("typicons"),
                svg_dir: Cow::Borrowed("src/svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Heroicons => PackageMetadata {
                short_name: Cow::Borrowed("hi"),
                package_name: Cow::Borrowed("Heroicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/refactoringui/heroicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.0.18"),
                        version: SemVer {
                            major: 2,
                            minor: 0,
                            patch: 18,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("heroicons"),
                svg_dir: Cow::Borrowed("src"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            // NOTE: we cannot update to a version later than 2.1.1 because of the non-permissive
            // license of the newer versions.
            PackageType::CssGg => PackageMetadata {
                short_name: Cow::Borrowed("cg"),
                package_name: Cow::Borrowed("css.gg"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/astrit/css.gg"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("2.1.1"),
                        version: SemVer {
                            major: 2,
                            minor: 1,
                            patch: 1,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("css.gg"),
                svg_dir: Cow::Borrowed("icons/svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::TablerIcons => PackageMetadata {
                short_name: Cow::Borrowed("tb"),
                package_name: Cow::Borrowed("Tabler Icons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/tabler/tabler-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v2.34.0"),
                        version: SemVer {
                            major: 2,
                            minor: 34,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("tabler-icons"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::GithubOcticons => PackageMetadata {
                short_name: Cow::Borrowed("oc"),
                package_name: Cow::Borrowed("Github Octicons"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/primer/octicons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v19.7.0"),
                        version: SemVer {
                            major: 19,
                            minor: 7,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("octicons"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Lucide => PackageMetadata {
                short_name: Cow::Borrowed("lu"),
                package_name: Cow::Borrowed("Lucide"),
                licenses: &[Cow::Borrowed("ISC")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/lucide-icons/lucide"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v0.265.0"),
                        version: SemVer {
                            major: 0,
                            minor: 265,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("lucide"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::Charm => PackageMetadata {
                short_name: Cow::Borrowed("ch"),
                package_name: Cow::Borrowed("Charm"),
                licenses: &[Cow::Borrowed("MIT")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/jaynewey/charm-icons"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v0.18.0"),
                        version: SemVer {
                            major: 0,
                            minor: 18,
                            patch: 0,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: Cow::Borrowed("charm"),
                svg_dir: Cow::Borrowed("icons"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
            PackageType::MaterialDesignIcons => PackageMetadata {
                short_name: Cow::Borrowed("mdi"),
                package_name: Cow::Borrowed("Material Design Icons"),
                licenses: &[Cow::Borrowed("Apache 2.0")],
                source: PackageSource::Git {
                    url: Cow::Borrowed("https://github.com/Templarian/MaterialDesign-SVG"),
                    target: GitTarget::Tag {
                        name: Cow::Borrowed("v7.4.47"),
                        version: SemVer {
                            major: 7,
                            minor: 4,
                            patch: 47,
                            prerelease: None,
                            build: None,
                        },
                    },
                },
                download_dir: "MaterialDesign-SVG".into(),
                svg_dir: Cow::Borrowed("svg"),
                crate_version: SemVer {
                    major: 0,
                    minor: 0,
                    patch: 10,
                    prerelease: None,
                    build: None,
                },
            },
        }
    }
}
