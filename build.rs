fn main() {
    println!("cargo:rustc-link-lib=dylib=mini_client");
    println!("cargo:rustc-link-search=all=libs");
}
