use bootimage::{builder, Config};
use std::env;
use std::path::PathBuf;

fn main() {
    // Get the output directory for build artifacts
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Get the manifest directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // Set up the configuration for the bootloader
    let config = Config::default();
    let args = vec!["--target", "x86_64-fem_dos.json"];
    let quiet = false;

    // Initialize the builder for building the bootable image
    let mut builder = builder::Builder::new(manifest_dir);

    // Build the bootable image using the config and target arguments
    builder.build_kernel(&out_dir, args, &config, quiet).unwrap();
    println!("Bootable image has been built!");
}
