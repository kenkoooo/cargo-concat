use cargo_concat::concat_source;
use std::fs::{read_to_string, write};

#[test]
fn test_concat_source() {
    let source = concat_source("./test-project/Cargo.toml", None).unwrap();
    let source = format_file(&source);
    let expected = read_to_string("./test-project/out.rs").unwrap();

    let parsed_source = syn::parse_file(&source).unwrap();
    assert_eq!(
        syn::parse_file(&expected).unwrap(),
        parsed_source,
        "{}",
        quote::quote!(#parsed_source)
    );
}

fn format_file(source_string: &str) -> String {
    let path = "/tmp/tmp.rs";
    write(path, source_string).unwrap();

    std::process::Command::new("rustfmt")
        .arg(path)
        .output()
        .unwrap();
    read_to_string(path).unwrap()
}
