use std::{
    env,
    io::{stdout, BufWriter, Write},
    path::Path,
    str::FromStr,
};

use anyhow::Context;

macro_rules! stringnify_list {
    ($($item:expr),*) => {
        &[
            $(
                stringify!($item),
            )*
        ]
    };
}

const ICONS: &[&str] = stringnify_list!(include!("ICON-LIST.txt"));

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-env-changed=ICONDATA_INCLUDE_ALL");
    if let Ok(incl_all) = env::var("ICONDATA_INCLUDE_ALL") {
        if incl_all == "1" || incl_all.to_lowercase() == "true" {
            println!("cargo:rustc-cfg=icondata_include_all");
            return Ok(());
        }
    }

    println!("cargo:rerun-if-env-changed=ICONDATA_MANIFEST_PATH");
    let icondata_file = env::var("ICONDATA_MANIFEST_PATH").context("icondata file path not set")?;

    println!("cargo:rerun-if-changed={}", &icondata_file);
    println!("cargo:warning={}", icondata_file);

    if !Path::new(&icondata_file).exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(icondata_file)
        .context("could not open Icondata.toml for reading")?;
    let toml_table = toml::Table::from_str(&content).context("could not parse Icondata.toml")?;
    if let Some(incl_all) = toml_table.get("include-all") {
        if incl_all.as_bool().context("include-all is not a boolean")? {
            println!("cargo:rustc-cfg=icondata_include_all");
            return Ok(());
        }
    }

    let mut out_buf = BufWriter::new(stdout().lock());

    if let Some(icons) = toml_table.get("icons") {
        let icons = icons.as_array().context("icons is not an array")?;
        icons
            .iter()
            .map(|value| value.as_str().context("icon is not a string"))
            .filter(|icon| {
                if let Ok(icon) = icon {
                    return icon.starts_with("Fa");
                }
                return true;
            })
            .try_for_each(|icon| -> anyhow::Result<()> {
                let icon = &icon?;
                if ICONS.contains(icon) {
                    writeln!(&mut out_buf, "cargo:rustc-cfg={}", icon)?;
                } else {
                    println!("cargo:warning=unknown icon found: {}", icon);
                }

                Ok(())
            })?;
    }

    out_buf.flush()?;
    Ok(())
}