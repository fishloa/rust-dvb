# Short Event Descriptor (tag 0x4D)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.37
**Parser file:** `crates/dvb_si/src/descriptors/short_event.rs`
**Rust struct:** `ShortEventDescriptor<'a>`

## Tables

### Table 92 — Service move descriptor
_PDF pages 102-102 (§6.2.37)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_move_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| new_original_network_id | 16 | uimsbf |
| new_transport_stream_id | 16 | uimsbf |
| new_service_id |  |  |
| } |  |  |

### Table 93 — Short event descriptor
_PDF pages 102-102 (§6.2.37)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| short_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| name_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| char | 8 | uimsbf |
| } |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| text_char |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.37, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
