use anyhow::Result;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn get_target_source_path(metadata: &Metadata, target: Option<&str>) -> Result<PathBuf> {
    let mut targets = metadata
        .packages
        .iter()
        .flat_map(|package| package.targets.iter())
        .filter(|target| {
            target.kind.contains(&"bin".to_string())
                && target.crate_types.contains(&"bin".to_string())
        });

    match target {
        Some(target) => targets
            .find(|t| t.name.as_str() == target)
            .map(|target| target.src_path.clone())
            .ok_or_else(|| anyhow::anyhow!("target={} is not found.", target)),
        None => {
            let first = targets.next();
            let second = targets.next();
            match first {
                Some(first) => {
                    if second.is_some() {
                        Err(anyhow::anyhow!(
                            "The project has multiple targets. Please specify one."
                        ))
                    } else {
                        Ok(first.src_path.clone())
                    }
                }
                None => Err(anyhow::anyhow!(
                    "The project has no target. Please create one or more."
                )),
            }
        }
    }
}

fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<()> {
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();
    let source_path = get_target_source_path(&metadata, None)?;
    let source_code = read_file(&source_path)?;
    let syn::File { items, .. } = syn::parse_file(&source_code)?;
    println!("{:#?}", items);

    Ok(())
}
