extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let pkg_name = "soundtouch";

    let mut builder = bindgen::Builder::default().header("wrapper.hpp");

    match pkg_config::probe_library(pkg_name) {
        Ok(library) => {
            for include_path in &library.include_paths {
                builder = builder.clang_args(&[
                    "-I",
                    include_path.to_str().expect(&format!(
                        "extract {} include path \"{:?}\" to str",
                        pkg_name, include_path,
                    )),
                ]);
            }

            for (define_key, define_value_option) in &library.defines {
                builder = if let Some(define_value) = define_value_option {
                    builder.clang_args(&["-D", &format!("{}={}", define_key, define_value)])
                } else {
                    builder.clang_args(&["-D", define_key])
                }
            }
        }
        Err(_) => {
            println!("cargo:rustc-link-lib=SoundTouch");
        }
    }

    let bindings = builder
        .whitelist_type("soundtouch::SoundTouch")
        .generate()
        .expect(&format!("generate {} bindings", pkg_name));
    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("environment variable `OUT_DIR' exists"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect(&format!("write {} bindings file", pkg_name));
}
