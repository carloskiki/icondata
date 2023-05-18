use anyhow::Result;
use clap::{command, Parser};
use icon::SvgIcon;
use package::Downloaded;
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::{Format, Pretty};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, Registry};

use crate::dirs::base_repo::BaseRepo;
use crate::dirs::boilerplate::Boilerplate;
use crate::dirs::icon_index::IconIndex;
use crate::dirs::icon_library::IconLibrary;
use crate::package::Package;

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

static ICONS: Arc<Mutex<Vec<SvgIcon>>> = Arc::new(Mutex::new(Vec::new()));
static PACKAGES: Arc<Mutex<Vec<Package<Downloaded>>>> = Arc::new(Mutex::new(Vec::new()));

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing(tracing::level_filters::LevelFilter::INFO);

    assert_paths();

    let args: BuildArgs = BuildArgs::parse();
    info!(?args, "Parsed program arguments.");

    let start = time::OffsetDateTime::now_utc();

    info!("Downloading all packages.");
    let handles = Package::all()
        .into_iter()
        .map(|package| {
            tokio::spawn(async move {
                if args.clean {
                    package.remove().await?;
                }

                let mut icons = Arc::clone(&ICONS).lock()?;
                // Download the package.
                let package_type = package.ty;
                let package = package.download(&mut icons).await.map_err(|err| {
                    error!(
                        ?package_type,
                        ?err,
                        "Downloading the package failed unexpectedly."
                    );
                    err
                })?;

                let mut packages = Arc::clone(&PACKAGES).lock()?;
                packages.push(package);

                Ok::<(), anyhow::Error>(package)
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

    info!("Generating individual icon crates.");
    todo!();

    let mut base_path = path::build_crate("");
    base_path.pop();
    let base_repo = BaseRepo::new(base_path);
    base_repo.generate().await?;

    let boilerplate_path = path::library_crate("boilerplate", "");
    let mut boilerplate_dir = Boilerplate::new(boilerplate_path);
    boilerplate_dir.generate(&libs).await?;

    let end = time::OffsetDateTime::now_utc();
    info!(
        took = format!("{}s", (end - start).whole_seconds()),
        num_libs, "Build successful!"
    );

    let icon_index = IconIndex::new(path::library_crate("icon_index", ""));
    icon_index.generate(&libs).await?;

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
    let icondata_crate_root = path::library_crate("icondata", "");
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
