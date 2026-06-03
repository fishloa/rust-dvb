# Mosaic (tag 0x51)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.21
**Parser file:** `crates/dvb_si/src/descriptors/0x51-mosaic.rs`
**Rust struct:** `MosaicDescriptor<'a>`

## Tables

### Table 70 — Country region id coding
_PDF pages 90-90 (§6.2.21)_

| country_region_id | Description |
|---|---|
| 0b00 0000 | no time zone extension used |
| 0b00 0001 | time zone 1 (most easterly region) |
| 0b00 0010 | time zone 2 |
| … | … |
| 0b11 1100 | time zone 60 |
| 0b11 1101 to 0b11 1111 | reserved for future use |

### Table 71 — Mosaic descriptor
_PDF pages 91-91 (§6.2.21)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| mosaic_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| mosaic_entry_point | 3 | uimsbf |
| number_of_horizontal_elementary_cells | 1 | bslbf |
| reserved_future_use | 3 | uimsbf |
| number_of_vertical_elementary_cells | 6 | uimsbf |
| for (i=0;i<N;i++) { | 7 | bslbf |
| logical_cell_id | 3 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| logical_cell_presentation_info | 2 | bslbf |
| elementary_cell_field_length | 6 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| reserved_future_use | 16 | uimsbf |
| elementary_cell_id | 16 | uimsbf |
| } | 16 | uimsbf |
| cell_linkage_info | 16 | uimsbf |
| if (cell_linkage_info == 0x01) { | 16 | uimsbf |
| bouquet_id | 16 | uimsbf |
| } | 16 | uimsbf |
| if (cell_linkage_info == 0x02) { | 16 | uimsbf |
| original_network_id | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| service_id | 16 | uimsbf |
| } |  |  |
| if (cell_linkage_info == 0x03) { |  |  |
| original_network_id |  |  |
| transport_stream_id |  |  |
| service_id |  |  |
| } |  |  |
| if (cell_linkage_info == 0x04) { |  |  |
| original_network_id |  |  |
| transport_stream_id |  |  |
| service_id |  |  |
| event_id |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 72 — Number of horizontal elementary cells coding
_PDF pages 91-91 (§6.2.21)_

| number_of_horizontal_elementary_cells | Description |
|---|---|
| 0 | one cell |
| 1 | two cells |
| 2 | three cells |
| 3 | four cells |
| 4 | five cells |
| 5 | six cells |
| 6 | seven cells |
| 7 | eight cells |

### Table 73 — Number of vertical elementary cells coding
_PDF pages 92-92 (§6.2.21)_

| number_of_vertical_elementary_cells | Description |
|---|---|
| 0 | one cell |
| 1 | two cells |
| 2 | three cells |
| 3 | four cells |
| 4 | five cells |
| 5 | six cells |
| 6 | seven cells |
| 7 | eight cells |

### Table 74 — Logical cell presentation info coding
_PDF pages 92-92 (§6.2.21)_

| logical_cell_presentation_info | Description |
|---|---|
| 0 | undefined |
| 1 | video |
| 2 | still picture (see note) |
| 3 | graphics/text |
| 4 to 7 | reserved for future use |
| NOTE: A coded still picture consists of a video sequence containing exactly one |  |
| coded picture which is intra-coded. |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.21, PDF pages 4-4. 5 tables / 34 rows reproduced verbatim._
