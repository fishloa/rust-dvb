# Network Information Table (table_id 0x40/0x41)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.1
**Parser file:** `crates/dvb_si/src/tables/nit.rs`
**Rust struct:** `Nit`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 3 — Network information section
_PDF pages 29-29 (§5.2.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| network_information_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 16 | uimsbf |
| network_id | 2 | bslbf |
| reserved | 5 | uimsbf |
| version_number | 1 | bslbf |
| current_next_indicator | 8 | uimsbf |
| section_number | 8 | uimsbf |
| last_section_number | 4 | bslbf |
| reserved_future_use | 12 | uimsbf |
| network_descriptors_length | 4 | bslbf |
| for (i=0;i<N;i++) { | 12 | uimsbf |
| descriptor() | 16 | uimsbf |
| } | 16 | uimsbf |
| reserved_future_use | 4 | bslbf |
| transport_stream_loop_length | 12 | uimsbf |
| for (i=0;i<N;i++) { | 32 | rpchof |
| transport_stream_id |  |  |
| original_network_id |  |  |
| reserved_future_use |  |  |
| transport_descriptors_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| descriptor() |  |  |
| } |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.1, PDF pages 3-3. 1 tables / 2 rows reproduced verbatim._
