use std::iter::FromIterator;
use syn::File;
use syn::Ident;
use syn::UseTree;

pub fn extract_modules(file: &File) -> Vec<Ident> {
    let mut idents = vec![];
    for item in file.items.iter() {
        if let syn::Item::Use(u) = item {
            match &u.tree {
                UseTree::Path(path) => {
                    idents.push(path.ident.clone());
                }
                UseTree::Name(name) => {
                    idents.push(name.ident.clone());
                }
                UseTree::Rename(rename) => {
                    idents.push(rename.ident.clone());
                }
                _ => log::warn!("{:?} is not supported yet.", u),
            }
        }
    }
    idents.sort();
    idents.dedup();
    Vec::from_iter(idents.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_modules() {
        let source_code = r"
                use std::env;
                use non_std::fun;
                use my_library::tool;
                use ::std::collections;
                use another_library;
                use glob_lib::*;
                use rename_lib as lib;
                
                fn main() {
                }
        ";
        let file = syn::parse_file(source_code).unwrap();
        let idents = extract_modules(&file);
        let idents = idents
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                "another_library",
                "glob_lib",
                "my_library",
                "non_std",
                "rename_lib",
                "std"
            ],
            idents
        );
    }
}
