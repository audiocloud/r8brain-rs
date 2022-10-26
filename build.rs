use std::env;

fn main() {
    let build_type = if cfg!(feature = "debug") {
        "Debug"
    } else {
        "Release"
    };

    let dst = cmake::Config::new(".").build_target("r8brain").build();

    println!("cargo:rustc-link-search=native={}/build/{}", dst.display(), build_type);
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=r8brain");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
