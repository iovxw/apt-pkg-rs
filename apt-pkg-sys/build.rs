extern crate gcc;

fn main() {
    let mut cfg = gcc::Config::new();

    cfg.cpp(true)
        .file("wrapper.cpp")
        .compile("libaptpkg.a");

    println!("cargo:rustc-link-lib=apt-pkg");
}
