use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let includes_output = Command::new("python3-config")
        .arg("--includes")
        .output()
        .expect("Failed to run python3-config — install python3 with dev headers");

    if !includes_output.status.success() {
        panic!("python3-config failed: {}", String::from_utf8_lossy(&includes_output.stderr));
    }

    let includes_str = String::from_utf8_lossy(&includes_output.stdout);

    let python_include = includes_str
        .split_whitespace()
        .filter(|s| s.starts_with("-I"))
        .next()
        .expect("python3-config --includes returned no -I flag")
        .trim_start_matches("-I");

    // Use header_contents with a synthetic wrapper so clang resolves
    // #include <Python.h> relative to the Python include directory
    // (and can find glibc headers through its own sysroot discovery).
    let bindings = bindgen::Builder::default()
        .header_contents(
            "python.h",
            "#include <Python.h>",
        )
         .clang_arg(format!("-I{}", python_include))
        // glibc's <math.h> defines FP_NAN, FP_INT_UPWARD, etc. both as enum
        // variants and as same-named macros, which bindgen emits twice.
        .blocklist_item("FP_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Error generating bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
