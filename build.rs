use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/avr/lib");
    println!("cargo:rustc-link-lib=c");

    let avr_include_path = "/usr/avr/include";
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I {avr_include_path}"))
        .clang_arg("-target")
        .clang_arg("avr")
        .clang_arg("-mmcu=atmega328p")
        .clang_arg("-D")
        .clang_arg("F_CPU=16000000")
        .generate_inline_functions(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
