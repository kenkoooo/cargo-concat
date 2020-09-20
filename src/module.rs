use anyhow::{Context, Result};
use proc_macro2::TokenTree;
use std::fs::read_to_string;
use std::path::Path;
use syn::{parse_file, File, Item};

pub fn concat_module<P: AsRef<Path> + std::fmt::Debug>(
    mod_file_path: P,
    is_mod_file: bool,
) -> Result<File> {
    log::info!("Parsing {:?}", mod_file_path);
    let content = read_to_string(&mod_file_path)?;
    let mut file = parse_file(&content)?;

    let mut cur_path = mod_file_path.as_ref().to_path_buf();
    if !is_mod_file {
        let ident = cur_path
            .file_stem()
            .with_context(|| format!("Invalid path operation for {:?}", cur_path))?
            .to_os_string();
        assert!(cur_path.pop(), "Invalid path operation for {:?}", cur_path);
        cur_path.push(ident);
        cur_path.push("dummy.rs");
    }

    for item in file.items.iter_mut() {
        if let Item::Mod(item) = item {
            if item.semi.is_none() {
                continue;
            }
            if let Some(file) = find_file_from_attr(&item.attrs) {
                let mut path = mod_file_path.as_ref().to_path_buf();
                assert!(path.pop(), "Invalid path operation.");
                path.push(file);
                log::info!("path={:?}", path);
                if path.exists() && path.is_file() {
                    item.semi = None;
                    let File { items, .. } = concat_module(path, false)?;
                    item.content = Some((syn::token::Brace::default(), items));
                    continue;
                }
            }

            let ident = item.ident.to_string();
            let mut path1 = cur_path.clone();
            assert!(path1.pop(), "Invalid path operation.");
            path1.push(&ident);
            assert!(path1.set_extension("rs"), "Invalid path operation");
            if path1.exists() && path1.is_file() {
                item.semi = None;
                let File { items, .. } = concat_module(path1, false)?;
                item.content = Some((syn::token::Brace::default(), items));
                continue;
            }

            let mut path2 = cur_path.clone();
            assert!(path2.pop(), "Invalid path operation.");
            path2.push(&ident);
            path2.push("mod.rs");
            if path2.exists() && path2.is_file() {
                item.semi = None;
                let File { items, .. } = concat_module(path2, true)?;
                item.content = Some((syn::token::Brace::default(), items));
                continue;
            }

            let path1 = path1
                .to_str()
                .with_context(|| format!("Failed to operate {:?}", path1))?;
            let path2 = path2
                .to_str()
                .with_context(|| format!("Failed to operate {:?}", path2))?;
            return Err(anyhow::anyhow!("Neither {} nor {} exists.", path1, path2));
        }
    }

    Ok(file)
}

fn find_file_from_attr(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr
            .path
            .segments
            .iter()
            .all(|seg| seg.ident.to_string().as_str() != "path")
        {
            continue;
        }

        let mut token = attr.tokens.clone().into_iter();
        let first = token.next();
        let second = token.next();
        if let (Some(TokenTree::Punct(_)), Some(TokenTree::Literal(lit))) = (first, second) {
            //FIXME
            let lit = lit.to_string();
            return Some(String::from(&lit[1..lit.len() - 1]));
        }
    }
    None
}
