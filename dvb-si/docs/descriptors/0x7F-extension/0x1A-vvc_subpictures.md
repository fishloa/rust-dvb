# Vvc Subpictures (extension sub-tag 0x1A)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.17
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x1A-vvc_subpictures.rs`
**Rust struct:** `VvcSubpicturesDescriptor<'a>`

## Tables

### Table 162 — Production disparity hint info
_PDF pages 148-148 (§6.4.17)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| production_disparity_hint_info() { | 12 | tcimsbf |
| video_max_disparity_hint | 12 | tcimsbf |
| video_min_disparity_hint |  |  |
| } |  |  |

### Table 162a — VVC subpictures descriptor
_PDF pages 149-149 (§6.4.17)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| vvc_subpictures_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 1 | bslbf |
| default_service_mode | 1 | bslbf |
| service_description_present | 6 | uimsbf |
| number_of_vvc_subpictures | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| component_tag | 5 | bslbf |
| vvc_subpicture_id | 3 | bslbf |
| } | 8 | uimsbf |
| reserved_zero_future_use | 8 | bslbf |
| processing_mode |  |  |
| if (service_description_present == 0b1) { |  |  |
| service_description_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 162b — Processing mode coding
_PDF pages 149-149 (§6.4.17)_

| processing_mode | Description |
|---|---|
| 0b000 | processing mode undefined |
| 0b001 | no bitstream processing necessary |
| 0b010 | merging of VVC subpictures into one bitstream necessary |
| 0b011 | reserved for future use |
| 0b100 | extraction of VVC subpictures from a bitstream necessary |
| 0b101 | reserved for future use |
| 0b110 | extraction and merging (replacement) of VVC subpictures necessary |
| 0b111 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.17, PDF pages 5-24. 3 tables / 13 rows reproduced verbatim._
