# Parental Rating (tag 0x55)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.28
**Parser file:** `crates/dvb_si/src/descriptors/0x55-parental_rating.rs`
**Rust struct:** `ParentalRatingDescriptor<'a>`

## Tables

### Table 80 — NVOD reference descriptor
_PDF pages 96-96 (§6.2.28)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| NVOD_reference_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| original_network_id |  |  |
| service_id |  |  |
| } |  |  |
| } |  |  |

### Table 81 — Network name descriptor
_PDF pages 96-96 (§6.2.28)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| network_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.28, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
