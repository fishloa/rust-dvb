# rtcp-packet 0.3.1 — 2026-08-30

Patch release: length-field overflow fix.

## Fixed

- **RTCP length-field u16 truncation** (#973). Payloads > ~256 KB silently
  wrapped the length field via a bare `as u16` cast. Now uses
  `u16::try_from` and returns `Error::InvalidValue` on overflow.

## Compatibility

No breaking changes. MSRV 1.95.0.
