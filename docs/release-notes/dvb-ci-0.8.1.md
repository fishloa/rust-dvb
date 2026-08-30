# dvb-ci 0.8.1 — 2026-08-30

Patch release: PID validation fix.

## Fixed

- **Silent 13-bit PID truncation** (#972). `CaPmtReply::serialize_into` and
  `host_control::Replace::serialize_into` silently masked PID values > 0x1FFF
  to 13 bits. Now rejects out-of-range PIDs with `Error::InvalidObject`.

## Compatibility

No breaking changes. MSRV 1.95.0.
