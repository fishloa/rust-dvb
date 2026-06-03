# Annex G — DTS Audio SI Implementation

**Spec:** ETSI EN 300 468 v1.19.1 Annex G (normative)

Defines SI signalling for DTS and DTS-HD audio in DVB systems.
`component_descriptor` with `stream_content=0x07` maps to DTS audio modes (Table G.1).
`DTS_descriptor` (tag 0x7B) and `DTS-HD_audio_stream_descriptor` (extension 0x0E) carry DTS parameters.

## Cross-references

- `0x7B-dts.md`
- `0x7F-extension/0x0E-dts_hd_audio_stream.md`
- `0x50-component.md` (stream_content=0x07)
