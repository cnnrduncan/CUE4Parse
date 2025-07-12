use std::env;
use std::path::PathBuf;

fn main() {
    // Only build native bindings if the native-lib feature is enabled
    if cfg!(feature = "native-lib") {
        // Tell cargo to look for shared libraries in the specified directory
        println!("cargo:rustc-link-search=native=../CUE4Parse-Natives/bin/Release");
        println!("cargo:rustc-link-search=native=../CUE4Parse-Natives/bin/Debug");
        
        // Link to the CUE4Parse-Natives library
        println!("cargo:rustc-link-lib=dylib=CUE4Parse-Natives");
        
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=wrapper.h");
        println!("cargo:rerun-if-changed=../CUE4Parse-Natives/cue4parse_c_api.h");
        println!("cargo:rerun-if-changed=../CUE4Parse-Natives/common/Framework.h");

        // Generate bindings only for the native library functions that exist
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate bindings for
            .header("wrapper.h")
            // Tell bindgen to look for headers in the CUE4Parse-Natives directory
            .clang_arg("-I../CUE4Parse-Natives")
            .clang_arg("-I../CUE4Parse-Natives/common")
            // Only generate bindings for functions that actually exist
            .allowlist_function("IsFeatureAvailable")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings
            .generate()
            // Unwrap the Result and panic on failure
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
