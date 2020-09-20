use anyhow::{Context, Result};
use cargo_metadata::Metadata;
use std::path::PathBuf;

pub fn get_module_lib_file(metadata: &Metadata, module_name: &str) -> Result<PathBuf> {
    let package = metadata
        .packages
        .iter()
        .find(|pkg| &pkg.name.replace("-", "_") == module_name)
        .with_context(|| format!("{:?} is not found.", module_name))?;
    let target = package
        .targets
        .iter()
        .find(|target| {
            target.kind.contains(&"lib".to_string())
                && target.crate_types.contains(&"lib".to_string())
        })
        .with_context(|| format!("Package {:?} doesn't have a target.", package))?;
    Ok(target.src_path.clone())
}
