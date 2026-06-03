# Image Icon (extension sub-tag 0x00)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.7
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x00-image_icon.rs`
**Rust struct:** `ImageIconDescriptor<'a>`

## Tables

### Table 144f — Postamble PLI
_PDF pages 134-134 (§6.4.7)_

| PLI | Description |
|---|---|
| 0 | Identical to the Superframe PLI |
| 1 to 3 | Reserved |
| 4 | L= 180 symbols |
| 5 | L=360 symbols |
| 6 | L= 900 symbols |
| 7 | L = 90 symbols |

### Table 145 — Image icon descriptor
_PDF pages 135-135 (§6.4.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| image_icon_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 4 | uimsbf |
| descriptor_number | 4 | uimsbf |
| last_descriptor_number | 5 | uimsbf |
| reserved_future_use | 3 | uimsbf |
| icon_id | 2 | uimsbf |
| if (descriptor_number == 0x00) { | 1 | bslbf |
| icon_transport_mode | 3 | uimsbf |
| position_flag | 2 | bslbf |
| if (position_flag == 0b1 | 12 | uimsbf |
| coordinate_system | 12 | uimsbf |
| reserved_future_use | 5 | bslbf |
| icon_horizontal_origin | 8 | uimsbf |
| icon_vertical_origin | 8 | uimsbf |
| } else { | 8 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| } | 8 | uimsbf |
| icon_type_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| icon_type_char | 8 | uimsbf |
| } |  |  |
| if (icon_transport_mode == 0) { |  |  |
| icon_data_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| icon_data_byte |  |  |
| } |  |  |
| } else if (icon_transport_mode == 1) { |  |  |
| url_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| url_char |  |  |
| } |  |  |
| } |  |  |
| } else { |  |  |
| icon_data_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| icon_data_byte |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.7, PDF pages 5-24. 2 tables / 9 rows reproduced verbatim._
