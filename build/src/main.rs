use anyhow::{anyhow, Result};
use clap::{command, Parser};
use icon::SvgIcon;
use package::Downloaded;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use crate::dirs::{LibType, Library};
use crate::package::Package;
use once_cell::sync::OnceCell;

mod dirs;
mod feature;
mod fs;
mod git;
mod icon;
mod package;
mod path;
mod sem_ver;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct BuildArgs {
    /// Clear downloads and re-download.
    #[arg(long, default_value_t = false)]
    clean: bool,
}

static PACKAGES: OnceCell<Packages> = OnceCell::new();

pub struct Packages {
    pub(crate) icons: Vec<SvgIcon>,
    pub packages: Vec<Package<Downloaded>>,
}

impl Packages {
    pub fn get() -> Result<&'static Packages> {
        PACKAGES
            .get()
            .ok_or(anyhow!("Packages not initialized yet."))
    }

    pub(crate) fn set(icons: Vec<SvgIcon>, packages: Vec<Package<Downloaded>>) -> Result<()> {
        PACKAGES.set(Packages { icons, packages }).or(Err(anyhow!(
            "Could not set value because it was already set."
        )))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    assert_paths();

    let args: BuildArgs = BuildArgs::parse();
    info!(?args, "Parsed program arguments.");

    let start = time::OffsetDateTime::now_utc();

    let icons: Arc<Mutex<Vec<SvgIcon>>> = Arc::new(Mutex::new(Vec::new()));
    let packages: Arc<Mutex<Vec<Package<Downloaded>>>> = Arc::new(Mutex::new(Vec::new()));

    info!("Downloading all packages.");
    let handles = Package::all()
        .into_iter()
        .map(|package| {
            let icons = Arc::clone(&icons);
            let packages = Arc::clone(&packages);

            tokio::spawn(async move {
                if args.clean {
                    package.remove().await?;
                }

                // Download the package.
                let mut icons = icons.lock().await;
                let package_type = package.ty;
                let package = package.download(&mut icons).await.map_err(|err| {
                    error!(
                        ?package_type,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                packages.lock().await.push(package);

                Ok::<(), anyhow::Error>(())
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        match handle.await.unwrap() {
            Ok(()) => (),
            Err(err) => {
                error!(?err, "Could not process package successfully.");
                return Err(err);
            }
        }
    }

    info!("Setting the package global variable.");
    Packages::set(
        Arc::try_unwrap(icons)
            .or(Err(anyhow!("More than one hold of icons ARC.")))?
            .into_inner(),
        Arc::try_unwrap(packages)
            .or(Err(anyhow!("More than one hold of packages ARC.")))?
            .into_inner(),
    )?;

    info!("Generating individual icon crates.");
    let handles = Packages::get()?.packages.iter().map(|package| {
        tokio::spawn(async move {
            let lib = Library::new(
                path::library_crate(format!("icondata_{}", package.meta.short_name)),
                LibType::IconLib(&package),
            );
            lib.generate().await
        })
    }).collect::<Vec<_>>();

    for handle in handles {
        match handle.await.unwrap() {
            Ok(()) => (),
            Err(err) => {
                error!(?err, "Could not process package successfully.");
                return Err(err);
            }
        }
    }

    let main_lib = Library::new(path::library_crate("icondata"), LibType::MainLib);
    main_lib.generate().await?;

    let boilerplate = Library::new(path::library_crate("boilerplate"), LibType::Boilerplate);
    boilerplate.generate().await?;

    let icon_index = Library::new(path::library_crate("icon_index"), LibType::IconIndex);
    icon_index.generate().await?;

    let num_icons = Packages::get()?.icons.len();
    let end = time::OffsetDateTime::now_utc();
    info!(
        took = format!("{}s", (end - start).whole_seconds()),
        num_icons, "Build successful!"
    );

    Ok(())
}

fn init_tracing(level: tracing::level_filters::LevelFilter) {
    fn build_log_filter(default_log_level: tracing::level_filters::LevelFilter) -> Targets {
        Targets::new().with_default(default_log_level)
    }

    fn build_tracing_subscriber_fmt_layer(
    ) -> tracing_subscriber::fmt::Layer<Registry, Pretty, Format<Pretty>> {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_thread_names(false)
            .with_thread_ids(false)
    }

    let fmt_layer_filtered =
        build_tracing_subscriber_fmt_layer().with_filter(build_log_filter(level));

    Registry::default().with(fmt_layer_filtered).init();
}

/// Simply tests that from the assumed repository root, both the "build" and "icondata" directories are visible.
/// This may prevent unwanted file operations in wrong directories.
fn assert_paths() {
    let build_crate_root = path::build_crate("");
    let icondata_crate_root = path::library_crate("icondata");
    info!(?build_crate_root, "Using");
    info!(?icondata_crate_root, "Using");

    assert_eq!(
        Some("build"),
        build_crate_root.file_name().and_then(|it| it.to_str())
    );
    assert_eq!(
        Some("icondata"),
        icondata_crate_root.file_name().and_then(|it| it.to_str())
    );
}
