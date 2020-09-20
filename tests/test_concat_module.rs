use cargo_concat::module;

#[test]
fn test_concat_module() {
    let result = module::concat_module("./test-project/src/lib.rs", true).unwrap();
    let expected = quote::quote! {
        pub mod mod1 {
            pub mod func1 {
                pub fn func1() {}
            }
        }
        pub mod mod2 {
            pub mod func1 {
                pub fn func1() {}
            }
        }
        pub mod mod3 {
            pub mod func1 {
                pub fn func1() {}
            }
            pub mod mod4 {
                pub mod func1 {
                    pub fn func1() {}
                }
            }
        }
        pub mod inner {
            pub fn a() {}
            pub fn b() {}
        }
    };
    assert_eq!(expected.to_string(), quote::quote!(#result).to_string());
}
