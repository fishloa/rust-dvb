# Selection Information Table (table_id 0x7F)

**Spec:** ETSI EN 300 468 v1.19.1 §7.1.2
**Parser file:** `crates/dvb_si/src/tables/sit.rs`
**Rust struct:** `Sit`

## Tables

### Table 163 — Discontinuity information section
_PDF pages 153-153 (§7.1.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| discontinuity_information_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 1 | uimsbf |
| transition_flag | 7 | bslbf |
| reserved_future_use |  |  |
| } |  |  |

### Table 164 — Selection information section
_PDF pages 154-154 (§7.1.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| selection_information_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 16 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 5 | uimsbf |
| version_number | 1 | bslbf |
| current_next_indicator | 8 | uimsbf |
| section_number | 8 | uimsbf |
| last_section_number | 4 | bslbf |
| reserved_future_use | 12 | uimsbf |
| transmission_info_descriptors_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| descriptor() | 3 | bslbf |
| } | 12 | uimsbf |
| for (i=0;i<N;i++) { | 32 | rpchof |
| service_id |  |  |
| reserved_future_use |  |  |
| running_status |  |  |
| service_descriptors_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| descriptor() |  |  |
| } |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §7.1.2, PDF pages 5-24. 2 tables / 4 rows reproduced verbatim._
