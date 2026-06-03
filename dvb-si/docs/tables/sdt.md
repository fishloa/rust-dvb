# Service Description Table (table_id 0x42/0x46)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.3
**Parser file:** `crates/dvb_si/src/tables/sdt.rs`
**Rust struct:** `Sdt`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 5 — Service description section
_PDF pages 32-32 (§5.2.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_description_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 16 | uimsbf |
| transport_stream_id | 2 | bslbf |
| reserved | 5 | uimsbf |
| version_number | 1 | bslbf |
| current_next_indicator | 8 | uimsbf |
| section_number | 8 | uimsbf |
| last_section_number | 16 | uimsbf |
| original_network_id | 8 | bslbf |
| reserved_future_use | 16 | uimsbf |
| for (i=0;i<N;i++) { | 6 | bslbf |
| service_id | 1 | bslbf |
| reserved_future_use | 1 | bslbf |
| EIT_schedule_flag | 3 | uimsbf |
| EIT_present_following_flag | 1 | bslbf |
| running_status | 12 | uimsbf |
| free_CA_mode | 32 | rpchof |
| descriptors_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| descriptor() |  |  |
| } |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

### Table 6 — Running status
_PDF pages 33-33 (§5.2.3)_

| running_status | Description |
|---|---|
| 0 | undefined |
| 1 | not running |
| 2 | starts in a few seconds (e.g. for video recording) |
| 3 | pausing |
| 4 | running |
| 5 | service off-air |
| 6 to 7 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.3, PDF pages 3-3. 2 tables / 10 rows reproduced verbatim._
