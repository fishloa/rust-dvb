# Running Status Table (table_id 0x71)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.7
**Parser file:** `crates/dvb_si/src/tables/rst.rs`
**Rust struct:** `Rst`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 9 — Time offset section
_PDF pages 37-37 (§5.2.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_offset_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 40 | bslbf |
| UTC_time | 4 | bslbf |
| reserved | 12 | uimsbf |
| descriptors_length | 32 | rpchof |
| for (i=0;i<N;i++) { |  |  |
| descriptor() |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.7, PDF pages 3-3. 1 tables / 2 rows reproduced verbatim._
