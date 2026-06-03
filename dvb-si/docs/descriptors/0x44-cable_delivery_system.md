# Cable Delivery System (tag 0x44)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.13.1
**Parser file:** `crates/dvb_si/src/descriptors/0x44-cable_delivery_system.rs`
**Rust struct:** `CableDeliverySystemDescriptor<'a>`

## Tables

### Table 32 — Data broadcast id descriptor
_PDF pages 72-72 (§6.2.13.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| data_broadcast_id_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| data_broadcast_id | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| selector_byte |  |  |
| } |  |  |
| } |  |  |

### Table 33 — Cable delivery system descriptor
_PDF pages 72-72 (§6.2.13.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cable_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | bslbf |
| frequency | 12 | bslbf |
| reserved_future_use | 4 | bslbf |
| FEC_outer | 8 | bslbf |
| modulation | 28 | bslbf |
| symbol_rate | 4 | bslbf |
| FEC_inner |  |  |
| } |  |  |

### Table 34 — Outer FEC scheme
_PDF pages 72-72 (§6.2.13.1)_

| FEC_outer | Description |
|---|---|
| 0b0000 | not defined |
| 0b0001 | no outer FEC coding |
| 0b0010 | (204,188) Reed-Solomon code (RS) |
| 0b0011 to 0b1111 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.13.1, PDF pages 4-4. 3 tables / 9 rows reproduced verbatim._
