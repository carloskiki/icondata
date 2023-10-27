use std::{
    env,
    io::{stdout, BufWriter, Write},
    path::Path,
    str::FromStr,
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-env-changed=ICONDATA_INCLUDE_ALL");

    if let Ok(incl_all) = env::var("ICONDATA_INCLUDE_ALL") {
        if incl_all == "1" || incl_all.to_lowercase() == "true" {
            println!("cargo:rustc-cfg=include_all");
            return Ok(());
        }
    }

    println!("cargo:rerun-if-env-changed=ICONDATA_MANIFEST_DIR");
    let icondata_manifest_dir = env::var("ICONDATA_MANIFEST_DIR").unwrap_or(env!("CARGO_MANIFEST_DIR").to_string());

    println!(
        "cargo:rerun-if-changed={}/Icondata.toml",
        &icondata_manifest_dir
    );

    let icondata_file = Path::new(&icondata_manifest_dir).join("Icondata.toml");

    if !icondata_file.exists() {
        println!("cargo:warning=Icondata.toml not found, skipping icondata generation. \n No icons will be included in the build.");
        return Ok(());
    }

    let content = std::fs::read_to_string(icondata_file)
        .context("could not open Icondata.toml for reading")?;
    let toml_table = toml::Table::from_str(&content).context("could not parse Icondata.toml")?;
    if let Some(incl_all) = toml_table.get("include-all") {
        if incl_all.as_bool().context("include-all is not a boolean")? {
            println!("cargo:rustc-cfg=include_all");
            return Ok(());
        }
    }

    let mut out_buf = BufWriter::new(stdout().lock());

    if let Some(icons) = toml_table.get("icons") {
        let icons = icons.as_array().context("icons is not an array")?;
        icons.iter().try_for_each(|icon| -> anyhow::Result<()> {
            let icon = icon.as_str().context("icon is not a string")?;
            writeln!(&mut out_buf, "cargo:rustc-cfg={}", icon)?;

            Ok(())
        })?;
    }

    Ok(())
}