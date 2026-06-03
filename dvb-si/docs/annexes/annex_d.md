# Annex D — AC-3 and Enhanced AC-3 SI Implementation

**Spec:** ETSI EN 300 468 v1.19.1 Annex D (normative)

Defines the SI implementation for AC-3 (Dolby Digital) and Enhanced AC-3 (Dolby Digital Plus)
audio in DVB systems. Specifies `AC-3_descriptor` (tag 0x6A) and `enhanced_AC-3_descriptor` (tag 0x7A) usage.

## Key points

- `component_descriptor` (0x50) with `stream_content=0x04` signals AC-3 audio modes (Table D.1)
- `component_descriptor` with `stream_content=0x04`, `component_type=0x80-0xFF` signals E-AC-3
- `AC-3_descriptor` / `enhanced_AC-3_descriptor` carry codec-specific parameters
- Both descriptors go in the PMT ES_info loop for the audio PID

## Cross-references

- `0x6A-ac3.md`, `0x7A-enhanced_ac3.md`
- `0x50-component.md` (stream_content=0x04 table)
- ETSI TS 101 154 for detailed codec requirements
