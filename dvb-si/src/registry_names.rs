//! Build-time-generated registry name lookups.
//!
//! This module `include!`s a file emitted by `build.rs` from the vendored
//! TSDuck `.names` data in `registries/`.  See `registries/README.md` for
//! provenance.
include!(concat!(env!("OUT_DIR"), "/registry_names.rs"));
