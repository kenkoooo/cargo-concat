pub mod extract;
pub mod metadata;
pub mod module;
pub mod target;

use anyhow::Result;
use cargo_metadata::{CargoOpt, MetadataCommand};
use std::fs::read_to_string;

pub fn concat_source(cargo_toml: &str, target: Option<&String>) -> Result<String> {
    let metadata = MetadataCommand::new()
        .manifest_path(cargo_toml)
        .features(CargoOpt::AllFeatures)
        .exec()?;
    let source_path = target::get_target_source_path(&metadata, target)?;
    log::info!("Target found: {:?}", source_path);

    let source_code = read_to_string(&source_path)?;
    let source_file = syn::parse_file(&source_code)?;

    let libs = extract::extract_modules(&source_file)
        .into_iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>();
    log::info!("Extracted modules: {:?}", libs);
    let mut paths = vec![];
    for lib_name in libs {
        if lib_name.as_str() == "std" || lib_name.as_str() == "crate" {
            continue;
        }

        match metadata::get_module_lib_file(&metadata, &lib_name) {
            Ok(lib_path) => paths.push((lib_name, lib_path)),
            Err(e) => log::warn!("{:?}", e),
        }
    }

    let mut file_content = quote::quote!(#source_file).to_string();

    for (lib_name, lib_path) in paths {
        let ident = quote::format_ident!("{}", lib_name);
        let file = module::concat_module(lib_path, true)?;
        file_content.push_str(quote::quote! {mod #ident {#file}}.to_string().as_str());
    }
    Ok(file_content)
}
