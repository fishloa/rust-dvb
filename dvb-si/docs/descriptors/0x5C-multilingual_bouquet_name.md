# Multilingual Bouquet Name (tag 0x5C)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.22
**Parser file:** `crates/dvb_si/src/descriptors/0x5C-multilingual_bouquet_name.rs`
**Rust struct:** `MultilingualBouquetNameDescriptor<'a>`

## Tables

### Table 75 — Cell linkage info coding
_PDF pages 93-93 (§6.2.22)_

| cell_linkage_info | Description |
|---|---|
| 0x00 | undefined |
| 0x01 | bouquet related |
| 0x02 | service related |
| 0x03 | other mosaic related |
| 0x04 | event related |
| 0x05 to 0xFF | reserved for future use |

### Table 76 — Multilingual bouquet name descriptor
_PDF pages 93-93 (§6.2.22)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_bouquet_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| name_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.22, PDF pages 4-4. 2 tables / 9 rows reproduced verbatim._
