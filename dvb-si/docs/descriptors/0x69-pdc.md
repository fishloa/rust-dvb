# Pdc (tag 0x69)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.30
**Parser file:** `crates/dvb_si/src/descriptors/0x69-pdc.rs`
**Rust struct:** `PdcDescriptor<'a>`

## Tables

### Table 82 — Parental rating descriptor
_PDF pages 97-97 (§6.2.30)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| parental_rating_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| country_code |  |  |
| rating |  |  |
| } |  |  |
| } |  |  |

### Table 83 — Parental rating coding
_PDF pages 97-97 (§6.2.30)_

| rating | Description |
|---|---|
| 0x00 | undefined |
| 0x01 to 0x0F | minimum age = rating + 3 years |
| 0x10 to 0xFF | defined by the broadcaster |

### Table 84 — PDC descriptor
_PDF pages 97-97 (§6.2.30)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| PDC_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | bslbf |
| reserved_future_use | 20 | bslbf |
| programme_identification_label |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.30, PDF pages 4-4. 3 tables / 8 rows reproduced verbatim._
