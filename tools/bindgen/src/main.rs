//! Regenerates `src/bindings.rs` from the Windows metadata bundled with
//! windows-bindgen. Run from the repository root:
//!
//!     cargo run --manifest-path tools/bindgen/Cargo.toml

fn main() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    std::env::set_current_dir(root).expect("repository root");
    windows_bindgen::bindgen(["--etc", "tools/bindgen/bindings.txt"]);
}
