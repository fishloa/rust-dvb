# Data Broadcast Id (tag 0x66)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.12
**Parser file:** `crates/dvb_si/src/descriptors/0x66-data_broadcast_id.rs`
**Rust struct:** `DataBroadcastIdDescriptor<'a>`

## Tables

### Table 31 — Data broadcast descriptor
_PDF pages 71-71 (§6.2.12)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| data_broadcast_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| data_broadcast_id | 8 | uimsbf |
| component_tag | 8 | uimsbf |
| selector_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 24 | bslbf |
| selector_byte | 8 | uimsbf |
| } | 8 | uimsbf |
| ISO_639_language_code |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.12, PDF pages 4-4. 1 tables / 2 rows reproduced verbatim._
