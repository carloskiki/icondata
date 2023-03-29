use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::{info, instrument, warn};

use crate::{
    feature::Feature,
    icon::{self, Category, IconSize, SvgIcon},
    package::Package,
    svg::ParsedSvg,
};

/// A directory to be searched, combined with:
///     - a list of categories valid for the contents of that directory and
///     - an icon size valid for icons inside the directory
#[derive(Debug)]
struct SearchDir {
    path: PathBuf,
    categories: Vec<Category>,
    icon_size: Option<IconSize>,
}

#[instrument(level = "info", skip(package), fields(package = ?package.ty))]
pub(crate) async fn get_icons(package: &Package) -> Result<Vec<SvgIcon>> {
    info!("Retrieving icon names...");
    let mut icons = Vec::new();

    let mut search_dirs = Vec::<SearchDir>::new();
    search_dirs.push(SearchDir {
        path: package.download_path().join(package.meta.svg_dir.as_ref()),
        categories: Vec::new(),
        icon_size: None,
    });

    while !search_dirs.is_empty() {
        let SearchDir {
            path,
            categories,
            icon_size,
        } = search_dirs.pop().expect("must exist");
        let mut dir_stream = tokio::fs::read_dir(&path).await?;
        while let Some(entry) = dir_stream.next_entry().await? {
            let entry_path = entry.path();

            if entry.file_type().await?.is_dir() {
                info!(additional_dir = ?entry_path, "Found additional directory.");

                // This dir must be searched as well. We consider is's name as being a "category" for the items contained in it.
                let file_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                let icon_size = icon_size.or_else(|| IconSize::from_str(&file_name).ok());

                let mut entry_cats = categories.clone();
                if package.ty.is_category(&file_name) {
                    entry_cats.push(Category(file_name));
                }
                search_dirs.push(SearchDir {
                    path: path.join(&entry_path),
                    categories: entry_cats,
                    icon_size,
                });

                continue;
            };

            match entry_path.extension() {
                Some(file_extension) => match file_extension.to_str() {
                    Some(file_extension) => match file_extension {
                        "svg" => icons.push(
                            svg_icon(&package, &entry_path, icon_size, categories.clone()).await?,
                        ),
                        _ => warn!(
                            ?entry_path,
                            file_extension, "Found file without svg extension. Ignoring it."
                        ),
                    },
                    None => {
                        warn!(
                                ?entry_path,
                                ?file_extension,
                                "Found file whose file_extension (&OsStr) could not be converted to a &str. Ignoring it."
                            );
                    }
                },
                None => warn!(?entry_path, "Found file without extension. Ignoring it."),
            };
        }
    }

    info!(num_icons = icons.len(), "Finished retrieving icon names.");
    Ok(icons)
}

async fn svg_icon(
    package: &Package,
    path: &Path,
    size: Option<IconSize>,
    mut categories: Vec<Category>,
) -> Result<SvgIcon> {
    let file_stem = path.file_stem().unwrap().to_string_lossy(); // TODO: Error handling\

    let (raw_name, size_from_name, cats_from_name) =
        icon::parse_raw_icon_name(package.ty, &file_stem);

    if let Some(mut cats_from_name) = cats_from_name {
        categories.append(&mut cats_from_name);
    }

    let feature = Feature {
        name: icon::feature_name(
            raw_name,
            size_from_name.or(size),
            &categories,
            &package.meta.short_name,
        ),
    };

    let svg = tokio::fs::read_to_string(path).await?;

    Ok(SvgIcon {
        svg: ParsedSvg::parse(svg.as_bytes())?,
        categories,
        component_name: feature.name.clone(), // TODO: Make clear why
        feature,
    })
}