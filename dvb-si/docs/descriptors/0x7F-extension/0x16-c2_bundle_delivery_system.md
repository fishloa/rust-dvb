# C2 Bundle Delivery System (extension sub-tag 0x16)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.6.4
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x16-c2_bundle_delivery_system.rs`
**Rust struct:** `C2BundleDeliverySystemDescriptor<'a>`

## Tables

### Table 139 — C2 bundle delivery system descriptor
_PDF pages 126-126 (§6.4.6.4)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| C2_bundle_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| plp_id | 32 | bslbf |
| data_slice_id | 2 | uimsbf |
| C2_System_tuning_frequency | 3 | bslbf |
| C2_System_tuning_frequency_type | 3 | bslbf |
| active_OFDM_symbol_duration | 1 | bslbf |
| guard_interval | 7 | bslbf |
| primary_channel |  |  |
| reserved_zero_future_use |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.6.4, PDF pages 5-24. 1 tables / 2 rows reproduced verbatim._
