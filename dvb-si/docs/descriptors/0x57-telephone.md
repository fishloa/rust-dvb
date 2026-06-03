# Telephone (tag 0x57)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.42
**Parser file:** `crates/dvb_si/src/descriptors/0x57-telephone.rs`
**Rust struct:** `TelephoneDescriptor<'a>`

## Tables

### Table 99 — Subtitling descriptor
_PDF pages 106-106 (§6.2.42)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| subtitling_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | bslbf |
| ISO_639_language_code | 16 | bslbf |
| subtitling_type | 16 | bslbf |
| composition_page_id |  |  |
| ancillary_page_id |  |  |
| } |  |  |
| } |  |  |

### Table 100 — Telephone descriptor
_PDF pages 107-107 (§6.2.42)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| telephone_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 2 | bslbf |
| reserved_future_use | 1 | bslbf |
| foreign_availability | 5 | uimsbf |
| connection_type | 1 | bslbf |
| reserved for future use | 2 | uimsbf |
| country_prefix_length | 3 | uimsbf |
| international_area_code_length | 2 | uimsbf |
| operator_code_length | 1 | bslbf |
| reserved for future use | 3 | uimsbf |
| national_area_code_length | 4 | uimsbf |
| core_number_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| country_prefix_char | 8 | uimsbf |
| } | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| international_area_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| operator_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| national_area_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| core_number_char |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.42, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
