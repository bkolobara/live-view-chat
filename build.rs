fn main() {
    // Changes in static files are not automatically detected by the Rust compiler.
    println!("cargo:rerun-if-changed=./static");
}
