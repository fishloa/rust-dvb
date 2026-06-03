# Cell Frequency Link (tag 0x6D)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.6
**Parser file:** `crates/dvb_si/src/descriptors/0x6D-cell_frequency_link.rs`
**Rust struct:** `CellFrequencyLinkDescriptor<'a>`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 21 — Bouquet name descriptor
_PDF pages 57-57 (§6.2.6)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| bouquet_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 22 — CA identifier descriptor
_PDF pages 57-57 (§6.2.6)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| CA_identifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| CA_system_id |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.6, PDF pages 3-3. 2 tables / 4 rows reproduced verbatim._
