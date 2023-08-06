use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Create a new cc builder.
    cc::Build::new()
        // Set the path of the C++ file to compile.
        .file("./src/cpp/src/lib.cpp")
        // Set the includd directory (e.g. where `lib.hpp` is).
        .include("./src/cpp/include/")
        // Tell cc this is C++.
        .cpp(true)
        // Compile to a library called `lib`.
        .compile("lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Set the path of `wrapper.hpp` (Similar to swig.i).
        .header("wrapper.hpp")
        // Tell Clang (used for C++ analysis) that this is C++ not C.
        .clang_args(vec!["-x", "c++"])
        // Tell Cargo information
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Add a `default` function based on C++ constructors.
        .derive_default(true)
        // Implement PartialEq.
        .impl_partialeq(true)
        // Generate the bindings
        .generate()
        // Panic on error.
        .expect("Unable to generate bindings");

    // Get the current build directory.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write the bindings to this directory.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
