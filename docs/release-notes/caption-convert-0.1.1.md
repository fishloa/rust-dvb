# caption-convert 0.1.1 — 2026-08-30

Patch release: three round-trip bugs fixed.

## Fixed

- **`parse_webvtt` rejected its own `write_segment` output** (#974). The
  `X-TIMESTAMP-MAP` header line emitted by HLS segment writing was misread
  as a cue identifier. Now recognised and skipped during header parsing.

- **`parse_srt` rejected UTF-8 BOM-prefixed files** (#975). BOM is now
  stripped before parsing, matching `parse_webvtt`'s existing behaviour.

- **`write_srt` emitted self-rejecting output** (#976) for cues with empty
  interior lines. The `\n\n` was interpreted as a block delimiter on
  re-parse. Empty lines within cue text are now skipped.

## Compatibility

No breaking changes. MSRV 1.95.0.
