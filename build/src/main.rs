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
    packages: Vec<Package<Downloaded>>,
}

impl Packages {
    pub fn get() -> Result<&'static [Package<Downloaded>]> {
        PACKAGES
            .get()
            .ok_or(anyhow!("Packages not initialized yet."))
            .map(|packages| packages.packages.as_ref())
    }

    pub fn get_icons() -> Result<impl Iterator<Item = &'static SvgIcon>> {
        Ok(Self::get()?.iter().flat_map(|package| package.icons().iter()))
    }

    pub fn set(packages: Vec<Package<Downloaded>>) -> Result<()> {
        PACKAGES.set(Packages { packages }).or(Err(anyhow!(
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

    let packages: Arc<Mutex<Vec<Package<Downloaded>>>> = Arc::new(Mutex::new(Vec::new()));

    info!("Downloading all packages.");
    let handles = Package::all()
        .into_iter()
        .map(|package| {
            let packages = Arc::clone(&packages);

            tokio::spawn(async move {
                if args.clean {
                    package.remove().await?;
                }

                // Download the package.
                let package_type = package.ty;
                let package = package.download().await.map_err(|err| {
                    error!(
                        ?package_type,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                info!("Generating icon crate.");
                let lib = Library::new(
                    path::library_crate(format!("icondata_{}", package.meta.short_name)),
                    LibType::IconLib(&package),
                );
                lib.generate().await?;

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
    let mut packages = Arc::try_unwrap(packages)
        .or(Err(anyhow!("More than one hold of packages ARC.")))?
        .into_inner();
    packages.sort_by(|a, b| a.meta.short_name.cmp(&b.meta.short_name));
    Packages::set(packages)?;

    info!("Generating main library.");
    let main_lib = Library::new(path::library_crate("icondata"), LibType::MainLib);
    main_lib.generate().await?;

    info!("Generating boilerplate directory.");
    let boilerplate = Library::new(path::library_crate("boilerplate"), LibType::Boilerplate);
    boilerplate.generate().await?;

    info!("Generating icon index.");
    let icon_index = Library::new(path::library_crate("icon_index"), LibType::IconIndex);
    icon_index.generate().await?;

    let num_libs = Packages::get()?.len();
    let end = time::OffsetDateTime::now_utc();
    info!(
        took = format!("{}ms", (end - start).whole_milliseconds()),
        num_libs, "Build successful!"
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
