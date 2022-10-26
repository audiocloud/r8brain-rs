use std::env;

fn main() {
    let build_type = if cfg!(feature = "debug") {
        "Debug"
    } else {
        "Release"
    };

    let dst = cmake::Config::new(".").build_target("r8brain").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    dst.push("build");
    println!("cargo:rustc-link-search=native={}", dst.display());
    dst.push(build_type);
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=r8brain");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
