# Target Region Name (extension sub-tag 0x0A)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.13
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x0A-target_region_name.rs`
**Rust struct:** `TargetRegionNameDescriptor<'a>`

## Tables

### Table 157 — Target region name descriptor
_PDF pages 144-144 (§6.4.13)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| target_region_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 24 | bslbf |
| country_code | 24 | bslbf |
| ISO_639_language_code | 2 | uimsbf |
| for (i=0;i<N;i++) { | 6 | uimsbf |
| region_depth | 8 | uimsbf |
| name_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| char | 16 | uimsbf |
| } |  |  |
| primary_region_code |  |  |
| if (region_depth >= 2) { |  |  |
| secondary_region_code |  |  |
| if (region_depth == 3) { |  |  |
| tertiary_region_code |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.13, PDF pages 5-24. 1 tables / 2 rows reproduced verbatim._
