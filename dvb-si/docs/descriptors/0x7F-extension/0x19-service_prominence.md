# Service Prominence (extension sub-tag 0x19)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.18
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x19-service_prominence.rs`
**Rust struct:** `ServiceProminenceDescriptor<'a>`

## Tables

### Table 162c — service_prominence_descriptor
_PDF pages 150-150 (§6.4.18)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_prominence_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| SOGI_list_length | 1 | bslbf |
| if (SOGI_list_length > 0) { | 1 | bslbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| SOGI_flag | 1 | bslbf |
| target_region_flag | 12 | uimsbf |
| service_flag | 16 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| SOGI_priority | 5 | bslbf |
| if (service_flag == 0b1) { | 1 | bslbf |
| service_id | 2 | uimsbf |
| } | 24 | bslbf |
| if (target_region_flag == 0b1) { | 8 | uimsbf |
| target_region_loop_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 16 | uimsbf |
| reserved_future_use | 8 | bslbf |
| country_code_flag |  |  |
| region_depth |  |  |
| if (country_code_flag == 0b1) { |  |  |
| country_code |  |  |
| } |  |  |
| if (region_depth >= 1) { |  |  |
| primary_region_code |  |  |
| if (region_depth >= 2) { |  |  |
| secondary_region_code |  |  |
| if (region_depth == 3) { |  |  |
| tertiary_region_code |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| for (i=0; i<N; i++) { |  |  |
| private_data_byte |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.18, PDF pages 5-24. 1 tables / 2 rows reproduced verbatim._
