# Stuffing Table (table_id 0x72)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.8
**Parser file:** `crates/dvb_si/src/tables/st.rs`
**Rust struct:** `St`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 10 — Running status section
_PDF pages 38-38 (§5.2.8)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| running_status_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| original_network_id | 16 | bslbf |
| service_id | 5 | uimsbf |
| event_id | 3 |  |
| reserved_future_use |  |  |
| running_status |  |  |
| } |  |  |
| } |  |  |

### Table 11 — Stuffing section
_PDF pages 38-38 (§5.2.8)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| stuffing_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| reserved_future_use | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| data_byte |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.8, PDF pages 3-3. 2 tables / 4 rows reproduced verbatim._
