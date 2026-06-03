# Time and Date Table (table_id 0x70)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.5
**Parser file:** `crates/dvb_si/src/tables/tdt.rs`
**Rust struct:** `Tdt`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 8 — Time and date section
_PDF pages 36-36 (§5.2.5)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_date_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 40 | bslbf |
| UTC_time |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.5, PDF pages 3-3. 1 tables / 2 rows reproduced verbatim._
