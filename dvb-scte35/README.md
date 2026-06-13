# dvb_scte35

[![crates.io](https://img.shields.io/crates/v/dvb-scte35.svg)](https://crates.io/crates/dvb-scte35)
[![docs.rs](https://img.shields.io/docsrs/dvb-scte35)](https://docs.rs/dvb-scte35)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE-MIT)

Spec-cited **ANSI/SCTE 35 2023r1** splice information (Digital Program
Insertion cueing) parser **and builder** in Rust, with the rust-dvb family's
symmetric `Parse`/`Serialize` round-trip discipline — every wire type
round-trips byte-for-byte.

> **Edition note.** This crate implements **ANSI/SCTE 35 2023r1**, the
> single-document edition of the standard. SCTE has since split the standard
> into **SCTE 35-1** (the message) and **SCTE 35-2** (XML/binary mappings); the
> binary `splice_info_section` syntax implemented here is unchanged.

## What is SCTE 35?

SCTE 35 is the cueing standard used across the cable / OTT industry to signal
ad-insertion (avail) and content-segmentation opportunities in an MPEG
transport stream. A `splice_info_section` (table_id `0xFC`) carries one splice
**command** (e.g. `splice_insert`, `time_signal`) plus a loop of splice
**descriptors** (e.g. `segmentation_descriptor`), trailed by an MPEG CRC-32.

## Coverage

| Area | Items | Status |
|---|---|---|
| Section | `splice_info_section` (`0xFC`): full §9.6 header, encryption flags (encrypted region kept raw), 33-bit `pts_adjustment`, 12-bit `tier`, CRC_32 | ✅ |
| Commands | `splice_null`, `splice_schedule`, `splice_insert`, `time_signal`, `bandwidth_reservation`, `private_command` (+ `splice_time` / `break_duration`) | ✅ |
| Descriptors | `avail`, `DTMF`, `segmentation`, `time`, `audio`; unknown tags fall through raw (lossless) | ✅ |
| Assignment tables | `segmentation_type_id` (Table 23), `segmentation_upid_type` (Table 22), `device_restrictions` (Table 21) as typed enums | ✅ |
| Decoded accessors | 90 kHz fields ⇄ `core::time::Duration`; 33-bit `pts_adjustment` carry-ignored wrap | ✅ |
| Dispatch | `AnyCommand` / `AnySpliceDescriptor` from a `declare_*!` list with drift tests | ✅ |
| serde | Serialize-only (no Deserialize), default-on `serde` feature | ✅ |

## Quick start

```rust
use dvb_scte35::{SpliceInfoSection, commands::{AnyCommand, TimeSignal}};
use dvb_scte35::time::SpliceTime;
use dvb_common::{Parse, Serialize};

// Build a time_signal() section and emit it.
let ts = TimeSignal { splice_time: SpliceTime::with_pts(0x0_0012_3456) };
let section = SpliceInfoSection::new_clear(AnyCommand::TimeSignal(ts), &[]);
let bytes = section.to_bytes();
assert_eq!(bytes[0], 0xFC); // table_id

// ...and parse it straight back (CRC verified on parse).
let parsed = SpliceInfoSection::parse(&bytes).unwrap();
assert!(matches!(parsed.clear.unwrap().command, AnyCommand::TimeSignal(_)));
```

## dvb-si integration

SCTE 35 sections ride on a PID the PMT labels with a registration descriptor
carrying the `"CUEI"` format_identifier — which [`dvb-si`](../dvb-si/) already
parses. Once you have the `0xFC` section bytes (e.g. from a dvb-si demux),
route them into `SpliceInfoSection::parse`.

## Spec grounding

The syntax tables and the `segmentation_type_id` / `segmentation_upid_type`
assignment tables are hand-transcribed in
[`dvb-scte35/docs/scte_35.md`](docs/scte_35.md); every module doc cites the SCTE
35 section, table and tag/command_type it implements. SCTE 35 is published by
SCTE at no cost.

## License

Licensed under either of MIT or Apache-2.0 at your option.
