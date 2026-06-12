//! Shared helpers for `dvb-tools` subcommands.

use std::process::ExitCode;

use dvb_si::ts::{TS_PACKET_SIZE, TS_SYNC_BYTE};

/// Read a file into memory. On `Err`, prints to stderr and returns `Err`.
pub(crate) fn read_file(path: &str, tool: &str) -> Result<Vec<u8>, ExitCode> {
    match std::fs::read(path) {
        Ok(d) => Ok(d),
        Err(e) => {
            eprintln!("{tool}: {path}: {e}");
            Err(ExitCode::FAILURE)
        }
    }
}

/// Yield every aligned 188-byte packet from `data`, skipping short tails and
/// non-sync-byte prefixes. Pulled into a helper so subcommands stay focused on
/// the per-table view rather than the TS framing loop.
pub(crate) fn for_each_packet(data: &[u8]) -> impl Iterator<Item = &[u8]> + '_ {
    data.chunks(TS_PACKET_SIZE)
        .filter(|c| c.len() == TS_PACKET_SIZE && c[0] == TS_SYNC_BYTE)
}
