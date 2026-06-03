# Vbi Data (tag 0x45)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.47
**Parser file:** `crates/dvb_si/src/descriptors/0x45-vbi_data.rs`
**Rust struct:** `VbiDataDescriptor<'a>`

## Tables

### Table 106 — VBI data descriptor
_PDF pages 110-110 (§6.2.47)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| VBI_data_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| data_service_id | 2 | bslbf |
| data_service_descriptor_length | 1 | bslbf |
| if (data_service_id == 0x01 | 5 | uimsbf |
| \|\| data_service_id == 0x02 | 8 | bslbf |
| \|\| data_service_id == 0x04 |  |  |
| \|\| data_service_id == 0x05 |  |  |
| \|\| data_service_id == 0x06 |  |  |
| \|\| data_service_id == 0x07) { |  |  |
| for (j=0;j<N;j++) { |  |  |
| reserved_future_use |  |  |
| field_parity |  |  |
| line_offset |  |  |
| } |  |  |
| } else { |  |  |
| for (j=0;j<N;j++) { |  |  |
| reserved_future_use |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 107 — Data service id coding
_PDF pages 110-110 (§6.2.47)_

| data_service_id | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | EBU teletext (requires additional teletext_descriptor) |
| 0x02 | inverted teletext |
| 0x03 | reserved for future use |
| 0x04 | Video Programme System (VPS) |
| 0x05 | Wide Screen Signalling (WSS) |
| 0x06 | closed captioning |
| 0x07 | monochrome 4:2:2 samples |
| 0x08 to 0xEF | reserved for future use |
| 0xF0 to 0xFF | user defined |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.47, PDF pages 4-4. 2 tables / 13 rows reproduced verbatim._
