# vss-snapshot

Safe Rust API for creating and managing Windows Volume Shadow Copy (VSS)
snapshots.

> **Status:** early development. The API is not usable yet.

## Why

VSS lets a backup tool read a consistent, point-in-time copy of a volume,
including files that are open or locked by other processes. Using it from Rust
today means hand-writing COM bindings, because the released
[`windows`](https://crates.io/crates/windows) crate does not include the
`IVssBackupComponents` interface that requesters need.

This crate aims to wrap that interface in a small, safe API: create a snapshot
of one or more volumes, read files through the snapshot's device path, and have
it cleaned up when you are done.

## Requirements

- Windows (Windows Server or a client edition with the VSS service running)
- Administrator rights, or the Backup Operators privilege

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
