//! Safe Rust API for creating and managing Windows Volume Shadow Copy (VSS)
//! snapshots.
//!
//! The crate only does anything on Windows. On other platforms it compiles to
//! an empty library, so it can sit in a cross-platform dependency tree.
