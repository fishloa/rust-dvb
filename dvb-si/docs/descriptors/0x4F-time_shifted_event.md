# Time Shifted Event (tag 0x4F)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.44
**Parser file:** `crates/dvb_si/src/descriptors/0x4F-time_shifted_event.rs`
**Rust struct:** `TimeShiftedEventDescriptor<'a>`

## Tables

### Table 101 — Teletext descriptor
_PDF pages 108-108 (§6.2.44)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| teletext_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 5 | uimsbf |
| ISO_639_language_code | 3 | uimsbf |
| teletext_type | 8 | uimsbf |
| teletext_magazine_number |  |  |
| teletext_page_number |  |  |
| } |  |  |
| } |  |  |

### Table 102 — Teletext type coding
_PDF pages 108-108 (§6.2.44)_

| teletext_type | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | initial teletext page |
| 0x02 | teletext subtitle page |
| 0x03 | additional information page |
| 0x04 | programme schedule page |
| 0x05 | teletext subtitle page for hearing impaired people |
| 0x06 to 0x1F | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.44, PDF pages 4-4. 2 tables / 10 rows reproduced verbatim._
