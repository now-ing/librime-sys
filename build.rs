extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // 设置环境变量
    env::set_var("LIBRIME_LIB_DIR", "/usr/local/lib");
    env::set_var("LIBRIME_INCLUDE_DIR", "/usr/local/include");
    env::set_var("DYLD_LIBRARY_PATH", "/usr/local/lib");

    let librime_include_dir = env::var("LIBRIME_INCLUDE_DIR").expect("LIBRIME_INCLUDE_DIR not set");
    let librime_lib_dir = env::var("LIBRIME_LIB_DIR").expect("LIBRIME_LIB_DIR not set");

    if librime_include_dir.is_empty() {
        panic!("LIBRIME_INCLUDE_DIR is empty");
    }

    if librime_lib_dir.is_empty() {
        panic!("LIBRIME_LIB_DIR is empty");
    }

    println!("cargo:rustc-link-search={}", librime_lib_dir);
    println!("cargo:rustc-link-lib=rime");

    if env::var("CARGO_FEATURE_SEPARATE_GEARS_LIB").is_ok() {
        println!("cargo:rustc-link-lib=rime-gears");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", librime_include_dir))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

