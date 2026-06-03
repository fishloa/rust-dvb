# Multilingual Network Name (tag 0x5B)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.24
**Parser file:** `crates/dvb_si/src/descriptors/0x5B-multilingual_network_name.rs`
**Rust struct:** `MultilingualNetworkNameDescriptor<'a>`

## Tables

### Table 77 — Multilingual component name descriptor
_PDF pages 94-94 (§6.2.24)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_component_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| component_tag | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| text_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 78 — Multilingual network name descriptor
_PDF pages 94-94 (§6.2.24)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_network_name_descriptor() { | 8 | uimsbf |
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
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.24, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
