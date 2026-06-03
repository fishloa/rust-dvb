# Service Move (tag 0x60)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.36
**Parser file:** `crates/dvb_si/src/descriptors/0x60-service_move.rs`
**Rust struct:** `ServiceMoveDescriptor<'a>`

## Tables

### Table 90 — Service availability descriptor
_PDF pages 101-101 (§6.2.36)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_availability_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| availability_flag | 7 | bslbf |
| reserved_future_use | 16 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| cell_id |  |  |
| } |  |  |
| } |  |  |

### Table 91 — Service list descriptor
_PDF pages 101-101 (§6.2.36)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_list_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| service_id |  |  |
| service_type |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.36, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
