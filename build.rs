fn main() {
    pyo3_build_config::add_extension_module_link_args();

    // the following doesn't seem to be needed though am leaving it in here for reference
    // in case someone needs an example of how to link to custom binaries.
    // println!(
    //     "cargo:rustc-link-arg=-Wl,-rpath,/Library/Developer/CommandLineTools/Library/Frameworks"
    // );
}
