fn main() {
    // No custom build steps needed - the woff crate handles everything
    println!("cargo:rerun-if-changed=build.rs");
}