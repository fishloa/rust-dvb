# Service Relocated (extension sub-tag 0x0B)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.10
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x0B-service_relocated.rs`
**Rust struct:** `ServiceRelocatedDescriptor<'a>`

## Tables

### Table 151 — Change type coding
_PDF pages 139-139 (§6.4.10)_

| change_type | Description |
|---|---|
| 0 | message only |
| 1 | minor - default |
| 2 | minor - multiplex removed |
| 3 | minor - service changed |
| 4 to 7 | reserved for future use for other minor changes |
| 8 | major - default |
| 9 | major - multiplex frequency changed |
| 10 | major - multiplex coverage changed |
| 11 | major - multiplex added |
| 12 to 15 | reserved for future use for other major changes |

### Table 152 — Service relocated descriptor
_PDF pages 139-139 (§6.4.10)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_relocated_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 16 | uimsbf |
| old_original_network_id | 16 | uimsbf |
| old_transport_stream_id | 16 | uimsbf |
| old_service_id |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.10, PDF pages 5-24. 2 tables / 13 rows reproduced verbatim._
