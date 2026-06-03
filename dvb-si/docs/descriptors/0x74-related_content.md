# Related Content (tag 0x74)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.26
**Parser file:** `crates/dvb_si/src/descriptors/0x74-related_content.rs`
**Rust struct:** `RelatedContentDescriptor<'a>`

## Tables

### Table 79 — Multilingual service name descriptor
_PDF pages 95-95 (§6.2.26)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_service_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| service_provider_name_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| char |  |  |
| } |  |  |
| service_name_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.26, PDF pages 4-4. 1 tables / 2 rows reproduced verbatim._
