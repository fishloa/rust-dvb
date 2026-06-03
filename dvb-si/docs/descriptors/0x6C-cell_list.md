# Cell List (tag 0x6C)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.7
**Parser file:** `crates/dvb_si/src/descriptors/0x6C-cell_list.rs`
**Rust struct:** `CellListDescriptor<'a>`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 23 — Cell frequency link descriptor
_PDF pages 58-58 (§6.2.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cell_frequency_link_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 32 | uimsbf |
| cell_id | 8 | uimsbf |
| frequency | 8 | uimsbf |
| subcell_info_loop_length | 32 | uimsbf |
| for (j=0;j<N;j++) { |  |  |
| cell_id_extension |  |  |
| transposer_frequency |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 24 — Cell list descriptor
_PDF pages 58-58 (§6.2.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cell_list_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| cell_id | 16 | uimsbf |
| cell_latitude | 12 | uimsbf |
| cell_longitude | 12 | uimsbf |
| cell_extent_of_latitude | 8 | uimsbf |
| cell_extent_of_longitude | 8 | uimsbf |
| subcell_info_loop_length | 16 | uimsbf |
| for (j=0;j<N;j++) { | 16 | uimsbf |
| cell_id_extension | 12 | uimsbf |
| subcell_latitude | 12 | uimsbf |
| subcell_longitude |  |  |
| subcell_extent_of_latitude |  |  |
| subcell_extent_of_longitude |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.7, PDF pages 3-3. 2 tables / 4 rows reproduced verbatim._
