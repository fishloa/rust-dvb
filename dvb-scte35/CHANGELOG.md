# Changelog

## [Unreleased]

### Added
- **New crate `dvb-scte35`** (#58) — ANSI/SCTE 35 2023r1 splice information
  (Digital Program Insertion cueing) parser **and** builder, with the
  workspace's symmetric `Parse`/`Serialize` round-trip discipline.
  - `SpliceInfoSection` (table_id `0xFC`): the full §9.6 header
    (protocol_version, sap_type, the encryption flags, 33-bit `pts_adjustment`,
    `cw_index`, 12-bit `tier`, `splice_command_length`/type, descriptor loop,
    CRC_32 via `dvb_common::crc32_mpeg2`). Encrypted sections are kept raw and
    round-trip losslessly; clear sections expose typed commands and descriptors.
  - Commands: `splice_null`, `splice_schedule`, `splice_insert`, `time_signal`,
    `bandwidth_reservation`, `private_command`, plus `splice_time()` /
    `break_duration()` — unified by `AnyCommand` with a raw fall-through for
    reserved command types.
  - Splice descriptors: `avail`, `DTMF`, `segmentation` (with the
    `SegmentationTypeId` / `SegmentationUpidType` / `DeviceRestrictions`
    assignment-table enums), `time`, `audio` — unified by `AnySpliceDescriptor`
    with a raw fall-through for unknown tags.
  - Decoded accessors (the 4.1.0 pattern): 90 kHz fields (`pts_time`,
    `break_duration`, `pts_adjustment`) ⇄ `core::time::Duration`, and the 33-bit
    `pts_adjustment` carry-ignored wrap (`pts_add_wrapping`).
  - Optional `serde` feature (default on), Serialize-only — mirrors the
    workspace posture (no Deserialize).
