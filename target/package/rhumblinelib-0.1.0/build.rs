extern crate cc;

fn main() {
    let src = ["src/C/rhumbline.cpp"];

    cc::Build::new()
    .files(src.iter())
    .compile("rhumblinelib");
    println!("cargo:rustc-link-lib=Geographic");
    println!("cargo:rustc-link-lib=stdc++");
}