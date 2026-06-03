# Extension (tag 0x7F)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.16
**Parser file:** `crates/dvb_si/src/descriptors/0x7F-extension.rs`
**Rust struct:** `ExtensionDescriptor<'a>`

## Tables

### Table 53 — Extended event descriptor
_PDF pages 79-79 (§6.2.16)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| extended_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | uimsbf |
| descriptor_number | 4 | uimsbf |
| last_descriptor_number | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| length_of_items | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| item_description_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| item_description_char | 8 | uimsbf |
| } | 8 | uimsbf |
| item_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| item_char |  |  |
| } |  |  |
| } |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| text_char |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.16, PDF pages 4-4. 1 tables / 2 rows reproduced verbatim._
