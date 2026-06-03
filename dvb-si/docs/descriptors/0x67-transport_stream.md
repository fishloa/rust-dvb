# Transport Stream (tag 0x67)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.46
**Parser file:** `crates/dvb_si/src/descriptors/0x67-transport_stream.rs`
**Rust struct:** `TransportStreamDescriptor<'a>`

## Tables

### Table 103 — Time shifted event descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_shifted_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| reference_service_id | 16 | uimsbf |
| reference_event_id |  |  |
| } |  |  |

### Table 104 — Time shifted service descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_shifted_service_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| reference_service_id |  |  |
| } |  |  |

### Table 105 — Transport stream descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| transport_stream_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| byte |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.46, PDF pages 4-4. 3 tables / 6 rows reproduced verbatim._
