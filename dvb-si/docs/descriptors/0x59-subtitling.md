# Subtitling (tag 0x59)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.41
**Parser file:** `crates/dvb_si/src/descriptors/0x59-subtitling.rs`
**Rust struct:** `SubtitlingDescriptor<'a>`

## Tables

### Table 97 — Stream identifier descriptor
_PDF pages 105-105 (§6.2.41)_

| sb_leak_rate | Leak rate (Mbit/s) |
|---|---|
| 60 | 108,0 |
| 61 to 63 | reserved for future use |

### Table 98 — Stuffing descriptor
_PDF pages 105-105 (§6.2.41)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| stream_identifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| component_tag |  |  |
| } |  |  |
| stuffing_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | bslbf |
| for (i=0;i<N;i++) { |  |  |
| stuffing_byte |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.41, PDF pages 4-4. 2 tables / 6 rows reproduced verbatim._
