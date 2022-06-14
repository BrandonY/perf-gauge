fn main() {
    // println!("cargo:rustc-link-search=all=./libs");
    // println!("cargo:rustc-link-lib=static=mini_client");
    println!("cargo:rustc-link-lib=dylib=mini_client");
    println!("cargo:rustc-link-search=all=/home/yarbrough_google_com/perf-gauge/libs");
    // println!("cargo:rustc-link-lib=dylib=mini_client");
    // println!("cargo:rustc-link-arg=-C");
}
