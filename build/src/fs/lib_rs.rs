use std::{io, path::PathBuf};

use anyhow::Result;
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use indoc::indoc;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use snafu::{prelude::*, Backtrace};
use tokio::io::AsyncWriteExt;
use tracing::{error, trace};

use crate::{
    dirs::{boilerplate::Boilerplate, icon_index::IconIndex, icon_library::IconLibrary},
    icon::SvgIcon,
    package::Package,
};

const DOCS: &[u8] = indoc! {r#"
            //! This crate provides a collection of icons in the form of SVG data
            //! and an enum to select them.
            //!
            //! ## Usage
            //!
            //! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
            //! identical.
            //!
            //! The enum implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
            //!
            "#}.as_bytes();

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("Unable to create file {path:?}"))]
    CreateFile {
        source: io::Error,
        path: PathBuf,
        backtrace: Backtrace,
    },
    #[snafu(display("Unable to parse TokenStream"))]
    ParseTokenStream {
        source: syn::Error,
        backtrace: Backtrace,
    },
}

#[derive(Debug)]
pub(crate) struct LibRs<T> {
    pub path: PathBuf,
    pub(crate) _phantom: std::marker::PhantomData<T>,
}

impl<T: std::fmt::Debug> LibRs<T> {
    pub(crate) async fn init(&self) -> Result<(), Error> {
        trace!(path = ?self.path, "Creating new lib.rs file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .with_context(|_| CreateFileSnafu {
                path: self.path.clone(),
            })?;
        Ok(())
    }

    /// Opens the file for appending thereby creating it if non-existent.
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open lib.rs file to append modules.");
                    err
                })?,
        ))
    }

    async fn write(&self, content: &[u8]) -> Result<()> {
        let mut writer = self.append().await?;
        writer.write_all(content).await?;
        writer.flush().await.map_err(|err| {
            error!(?err, "Could not flush lib.rs file after writing.");
            err
        })?;
        Ok(())
    }
}

impl LibRs<Boilerplate> {
    pub async fn write_lib_rs(&self) -> Result<()> {
        let reexports = Self::build_reexports()?;
        self.write(reexports.as_bytes()).await?;
        self.write("\n// specific crate code ... ".as_bytes())
            .await?;

        Ok(())
    }

    fn build_reexports() -> Result<String> {
        let packages = Package::all();
        let statements = packages.iter().map(|pack| {
            let lib_short_name = &pack.meta.short_name;
            let short_name_upper = lib_short_name.to_upper_camel_case();
            let lib_name_ident = format_ident!("icondata_{}", lib_short_name);
            quote! {
                #[cfg(feature = #short_name_upper)]
                pub use #lib_name_ident::*;
            }
        });
        let statements = quote! {
            #(#statements)*
        };
        let tokens_file = syn::parse2::<syn::File>(statements).context(ParseTokenStreamSnafu {})?;
        Ok(prettyplease::unparse(&tokens_file))
    }
}

impl LibRs<IconLibrary> {
    pub(crate) async fn write_lib_rs(&self, enum_name: &str, icons: &[SvgIcon]) -> Result<()> {
        let enum_code = Self::build_enum(enum_name, icons)?;
        let icon_data = Self::build_icon_data(enum_name, icons)?;
        self.write(DOCS).await?;
        self.write(enum_code.as_bytes()).await?;
        self.write(icon_data.as_bytes()).await?;

        Ok(())
    }

    fn build_enum(enum_name: &str, icons: &[SvgIcon]) -> Result<String> {
        let variants = icons.iter().map(|icon| {
            let feature_name = &icon.feature.name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            quote! {
                #[cfg(feature = #feature_name)]
                #feature_ident
            }
        });

        let enum_ident = Ident::new(enum_name, Span::call_site());

        let icon_enum = quote! {
            #[non_exhaustive]
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
            pub enum #enum_ident {
                #(#variants),*
            }
        };

        let tokens_file: syn::File = syn::parse2(icon_enum).map_err(|err| {
            error!(?err, "Error parsing enum to token stream");
            err
        })?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    fn build_icon_data(enum_name: &str, icons: &[SvgIcon]) -> Result<String> {
        struct Data {
            style: Option<String>,
            x: Option<String>,
            y: Option<String>,
            width: Option<String>,
            height: Option<String>,
            view_box: Option<String>,
            stroke_linecap: Option<String>,
            stroke_linejoin: Option<String>,
            stroke_width: Option<String>,
            stroke: Option<String>,
            fill: Option<String>,
            data: String,
            feature_name: String,
        }
        let icons = icons
            .iter()
            .map(|icon| Data {
                style: icon
                    .svg
                    .svg_attributes
                    .style
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                x: icon
                    .svg
                    .svg_attributes
                    .x
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                y: icon
                    .svg
                    .svg_attributes
                    .y
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                width: icon
                    .svg
                    .svg_attributes
                    .width
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                height: icon
                    .svg
                    .svg_attributes
                    .height
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                view_box: icon
                    .svg
                    .svg_attributes
                    .view_box
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_linecap: icon
                    .svg
                    .svg_attributes
                    .stroke_linecap
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_linejoin: icon
                    .svg
                    .svg_attributes
                    .stroke_linejoin
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke_width: icon
                    .svg
                    .svg_attributes
                    .stroke_width
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                stroke: icon
                    .svg
                    .svg_attributes
                    .stroke
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                fill: icon
                    .svg
                    .svg_attributes
                    .fill
                    .as_ref()
                    .map(|it| it.value.to_owned()),
                data: icon.svg.content.clone(),
                feature_name: icon.feature.name.to_owned(),
            })
            .collect::<Vec<_>>();

        let const_icon_data = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
            let const_data_name = feature_name.to_shouty_snake_case();
            let const_data_ident = Ident::new(&const_data_name, Span::call_site());

            fn quote_opt(value: &Option<String>) -> TokenStream {
                match value {
                    Some(value) => quote! { Some(#value) },
                    None => quote! { None },
                }
            }

            let style = quote_opt(&icon.style);
            let x = quote_opt(&icon.x);
            let y = quote_opt(&icon.y);
            let width = quote_opt(&icon.width);
            let height = quote_opt(&icon.height);
            let view_box = quote_opt(&icon.view_box);
            let stroke_linecap = quote_opt(&icon.stroke_linecap);
            let stroke_linejoin = quote_opt(&icon.stroke_linejoin);
            let stroke_width = quote_opt(&icon.stroke_width);
            let stroke = quote_opt(&icon.stroke);
            let fill = quote_opt(&icon.fill);
            let data = &icon.data;

            quote! {
                #[cfg(feature = #feature_name)]
                const #const_data_ident: icondata_core::IconData = icondata_core::IconData {
                    style: #style,
                    x: #x,
                    y: #y,
                    width: #width,
                    height: #height,
                    view_box: #view_box,
                    stroke_linecap: #stroke_linecap,
                    stroke_linejoin: #stroke_linejoin,
                    stroke_width: #stroke_width,
                    stroke: #stroke,
                    fill: #fill,
                    data: #data,
                };
            }
        });

        let match_arms = icons.iter().map(|icon| {
            let feature_name = &icon.feature_name;
            let feature_ident = Ident::new(feature_name.as_str(), Span::call_site());
            let enum_ident = Ident::new(enum_name, Span::call_site());
            let const_data_name = feature_name.to_shouty_snake_case();
            let const_data_ident = Ident::new(&const_data_name, Span::call_site());

            quote! {
                #[cfg(feature = #feature_name)]
                #enum_ident::#feature_ident => #const_data_ident
            }
        });

        let enum_ident = Ident::new(enum_name, Span::call_site());

        let enum_impl = quote! {
            #(#const_icon_data)*

            impl From<#enum_ident> for icondata_core::IconData {
                fn from(icon: #enum_ident) -> icondata_core::IconData {
                    match icon {
                        #(#match_arms),*
                    }
                }
            }
        };

        let tokens_file: syn::File = syn::parse2(enum_impl)?;
        Ok(prettyplease::unparse(&tokens_file))
    }
}

impl LibRs<IconIndex> {
    pub(crate) async fn write_lib_rs(&self, icon_libs: &[IconLibrary]) -> Result<()> {
        let mut file = self.append().await?;

        let imports = Self::imports();
        let all_icons_func = Self::generate_constants(icon_libs)?;

        file.write_all(imports.as_bytes()).await?;
        file.write_all(all_icons_func.as_bytes()).await?;

        file.flush().await.map_err(|err| {
            error!(?err, "Could not flush file after writing.");
            err
        })?;
        Ok(())
    }

    fn imports() -> String {
        indoc! {"
            use leptos_icons::*;
            use strum::{IntoEnumIterator, VariantNames};
            use once_cell::sync::Lazy;

            "}
        .to_string()
    }

    fn generate_constants(icon_libs: &[IconLibrary]) -> Result<String> {
        trace!("Generating all_icons function.");
        let (names, packages): (Vec<_>, Vec<_>)  = icon_libs.iter().map(|lib| {
            let enum_name_ident = Ident::new(&lib.enum_name(), Span::call_site());
            let names = quote!(#enum_name_ident::VARIANTS);
            let packages = quote!(#enum_name_ident::iter().map(|i| IconData::from(i)));
            (names, packages)
        }).unzip();

        trace!("Quoting Code.");
        let consts = quote! {
            pub const NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| [
                    #(#names),*
                ].concat()
            );

            pub static ALL_ICONS: Lazy<Vec<IconData>> = Lazy::new(|| {
                itertools::chain!{
                    #(#packages),*
                }.collect()
            });
        };

        let tokens_file: syn::File = syn::parse2(consts)?;
        Ok(prettyplease::unparse(&tokens_file))
    }

    pub(crate) async fn reset(&self) -> Result<()> {
        if self.path.exists() {
            trace!("Removing file.");
            tokio::fs::remove_file(&self.path).await?;
        }

        trace!("Creating new file.");
        self.init().await?;

        Ok(())
    }
}
