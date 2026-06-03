# Event Information Table (table_id 0x4E–0x6F)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.4
**Parser file:** `crates/dvb_si/src/tables/eit.rs`
**Rust struct:** `Eit`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 7 — Event information section
_PDF pages 34-34 (§5.2.4)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| event_information_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 16 | uimsbf |
| service_id | 2 | bslbf |
| reserved | 5 | uimsbf |
| version_number | 1 | bslbf |
| current_next_indicator | 8 | uimsbf |
| section_number | 8 | uimsbf |
| last_section_number | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| original_network_id | 8 | uimsbf |
| segment_last_section_number | 8 | uimsbf |
| last_table_id | 16 | uimsbf |
| for (i=0;i<N;i++) { | 40 | bslbf |
| event_id | 24 | uimsbf |
| start_time | 3 | uimsbf |
| duration | 1 | bslbf |
| running_status | 12 | uimsbf |
| free_CA_mode | 32 | rpchof |
| descriptors_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| descriptor() |  |  |
| } |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

### Table 7a — Examples of last_table_id values
_PDF pages 35-35 (§5.2.4)_

| Transmitted EITtable_id | Service | last_table_id |
|---|---|---|
| 0x4E | A | 0x4E |
| 0x4F | A | 0x4F |
| 0x50, 0x51 | A | 0x51 |
| 0x60, 0x61, 0x62 | A | 0x62 |
| 0x4E | B | 0x4E |
| 0x4F | B | 0x4F |
| 0x50, 0x51, 0x52, 0x53 | B | 0x53 |
| 0x60 | B | 0x60 |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.4, PDF pages 3-3. 2 tables / 11 rows reproduced verbatim._
