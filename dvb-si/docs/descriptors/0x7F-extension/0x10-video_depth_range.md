# Video Depth Range (extension sub-tag 0x10)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.16
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x10-video_depth_range.rs`
**Rust struct:** `VideoDepthRangeDescriptor<'a>`

## Tables

### Table 159 — URI linkage descriptor
_PDF pages 146-146 (§6.4.16.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| URI_linkage_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| uri_linkage_type | 8 | uimsbf |
| uri_length | 8 | bslbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| uri_char | 8 | bslbf |
| } |  |  |
| if ((uri_linkage_type == 0x00) |  |  |
| \|\| (uri_linkage_type == 0x01)) { |  |  |
| min_polling_interval |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| private_data_byte |  |  |
| } |  |  |
| } |  |  |

### Table 160 — Video depth range descriptor
_PDF pages 147-147 (§6.4.16.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| video_depth_range_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| range_type | 8 | bslbf |
| range_length |  |  |
| if (range_type == 0x0) { |  |  |
| production_disparity_hint_info() |  |  |
| } else if (range_type == 0x1) { |  |  |
| /* empty */ |  |  |
| } else { |  |  |
| for (j=0;j<N;j++) { |  |  |
| range_selector_byte |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 161 — Range type coding
_PDF pages 147-147 (§6.4.16.1)_

| range_type | Description |
|---|---|
| 0x00 | production disparity hint |
| 0x01 | multi-region disparity Supplemental Enhancement Information (SEI) present |
| 0x02 to 0xFF | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.16, PDF pages 5-24. 3 tables / 8 rows reproduced verbatim._
