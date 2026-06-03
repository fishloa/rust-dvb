# S2 Satellite Delivery System (tag 0x79)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.13.3
**Parser file:** `crates/dvb_si/src/descriptors/0x79-s2_satellite_delivery_system.rs`
**Rust struct:** `S2SatelliteDeliverySystemDescriptor<'a>`

## Tables

### Table 42 — S2 satellite delivery system descriptor
_PDF pages 75-75 (§6.2.13.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| S2_satellite_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| scrambling_sequence_selector | 1 | bslbf |
| multiple_input_stream_flag | 1 | bslbf |
| reserved_zero_future_use | 1 | bslbf |
| not_timeslice_flag | 2 | bslbf |
| reserved_future_use | 2 | uimsbf |
| TS_GS_mode | 6 | bslbf |
| if (scrambling_sequence_selector == 0b1) { | 18 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| scrambling_sequence_index | 8 | uimsbf |
| } |  |  |
| if (multiple_input_stream_flag == 0b1) { |  |  |
| input_stream_identifier |  |  |
| } |  |  |
| if (not_timeslice_flag == 0b0) { |  |  |
| timeslice_number |  |  |
| } |  |  |
| } |  |  |

### Table 43 — Coding of the TS GS mode
_PDF pages 75-75 (§6.2.13.3)_

| TS_GS_mode (see note) | Description |
|---|---|
| 0 | Generic Packetized |
| 1 | Generic Stream Encapsulation (GSE) |
| 2 | DVB transport stream |
| 3 | reserved for future use |
| NOTE: These values are different from similar assignments in table 3 in ETSI EN 302 307-1 [7]. |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.13.3, PDF pages 4-4. 2 tables / 8 rows reproduced verbatim._
