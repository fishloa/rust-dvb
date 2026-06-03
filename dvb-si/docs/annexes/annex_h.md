# Annex H — AAC Audio SI Implementation

**Spec:** ETSI EN 300 468 v1.19.1 Annex H (normative)

Defines SI signalling for AAC (HE-AAC / HE-AAC v2) audio in DVB systems.
`component_descriptor` with `stream_content=0x06` maps to AAC audio modes.
`AAC_descriptor` (tag 0x7C) carries codec-specific parameters.

## Cross-references

- `0x7C-aac.md`
- `0x50-component.md` (stream_content=0x06)
