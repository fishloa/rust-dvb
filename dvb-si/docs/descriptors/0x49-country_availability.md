# Country Availability (tag 0x49)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.10
**Parser file:** `crates/dvb_si/src/descriptors/0x49-country_availability.rs`
**Rust struct:** `CountryAvailabilityDescriptor<'a>`

## Tables

### Table 30 — Country availability descriptor
_PDF pages 70-70 (§6.2.10)_

| content_nibble_level_1 | content_nibble_level_2 | Description |  |
|---|---|---|---|
| 0xA | 0x5 | cooking |  |
| 0xA | 0x6 | advertisement/shopping |  |
| 0xA | 0x7 | gardening |  |
| 0xA | 0x8 to 0xE | reserved for future use |  |
| 0xA | 0xF | user defined |  |
| Special characteristics: |  |  |  |
| 0xB | 0x0 | original language |  |
| 0xB | 0x1 | black and white |  |
| 0xB | 0x2 | unpublished |  |
| 0xB | 0x3 | live broadcast |  |
| 0xB | 0x4 | plano-stereoscopic |  |
| 0xB | 0x5 | local or regional |  |
| 0xB | 0x6 to 0xE | reserved for future use |  |
| 0xB | 0xF | user defined |  |
| Adult: |  |  |  |
| 0xC | 0x0 | adult (general) |  |
| 0xC | 0x1 to 0xE | reserved for future use |  |
| 0xC | 0xF | user defined |  |
| Reserved for future use: |  |  |  |
| 0xD to 0xE | 0x0 to 0xF | reserved for future use |  |
| User defined: |  |  |  |
| 0xF | 0x0 to 0xF | user defined |  |
| Syntax | Number of bits | Identifier |  |
| country_availability_descriptor() { | 8 | uimsbf |  |
| descriptor_tag | 8 | uimsbf |  |
| descriptor_length | 1 | bslbf |  |
| country_availability_flag | 7 | bslbf |  |
| reserved_future_use | 24 | bslbf |  |
| for (i=0;i<N;i++) { |  |  |  |
| country_code |  |  |  |
| } |  |  |  |
| } |  |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.10, PDF pages 4-4. 1 tables / 25 rows reproduced verbatim._
