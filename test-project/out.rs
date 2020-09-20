use a::b;
use std;
use std::env;
use test_project::mod1;
mod a {
    pub mod b {}
}
fn main() {
    println!("Hello, world!");
}
mod test_project {
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
}
