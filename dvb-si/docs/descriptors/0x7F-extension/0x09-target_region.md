# Target Region (extension sub-tag 0x09)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.12
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x09-target_region.rs`
**Rust struct:** `TargetRegionDescriptor<'a>`

## Tables

### Table 156 — Target region descriptor
_PDF pages 143-143 (§6.4.12)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| target_region_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 24 | bslbf |
| country_code | 5 | bslbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| reserved_future_use | 2 | uimsbf |
| country_code_flag | 24 | bslbf |
| region_depth | 8 | uimsbf |
| if (country_code_flag == 0b1) { | 8 | uimsbf |
| country_code | 16 | uimsbf |
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

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.12, PDF pages 5-24. 1 tables / 2 rows reproduced verbatim._
