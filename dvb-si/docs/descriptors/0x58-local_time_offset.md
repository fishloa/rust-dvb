# Local Time Offset (tag 0x58)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.20
**Parser file:** `crates/dvb_si/src/descriptors/0x58-local_time_offset.rs`
**Rust struct:** `LocalTimeOffsetDescriptor<'a>`

## Tables

### Table 69 — Local time offset descriptor
_PDF pages 89-89 (§6.2.20)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| local_time_offset_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 6 | bslbf |
| country_code | 1 | bslbf |
| country_region_id | 1 | bslbf |
| reserved_future_use | 16 | bslbf |
| local_time_offset_polarity | 40 | bslbf |
| local_time_offset | 16 | bslbf |
| time_of_change |  |  |
| next_time_offset |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.20, PDF pages 4-4. 1 tables / 2 rows reproduced verbatim._
