# Adaptation Field Data (tag 0x70)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.1
**Parser file:** `crates/dvb_si/src/descriptors/0x70-adaptation_field_data.rs`
**Rust struct:** `AdaptationFieldDataDescriptor<'a>`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 13 — Adaptation field data descriptor
_PDF pages 54-54 (§6.2.1)_

| Descriptor | Tag | NIT |  | BAT |  | SDT |  | EIT |  | TOT |  | PMT |  | SIT |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  | value |  |  |  |  |  |  |  |  |  |  |  |  | (see note 1) |  |
| AAC_descriptor (see annex H) | 0x7C | - |  | - |  | - |  | - |  | - |  | ✓ |  | - |  |
| XAIT_location_descriptor (ETSI TS 102 727 [i.2]) | 0x7D | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  |
| FTA_content_management_descriptor | 0x7E | ✓ |  | ✓ |  | ✓ |  | ✓ |  | - |  | - |  | - |  |
| extension_descriptor (see note 4) | 0x7F | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  |
| user defined | 0x80 to |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | 0xFE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| reserved for future use | 0xFF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 1: Only found in Partial Transport Streams. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 2: Only in the TSDT. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 3: May also be located in the CAT (ISO/IEC 13818-1 [1]) and IP/MAC Notification Table (INT) (ETSI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| EN 301 192 [3]). |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 4: See also clause 6.3 and clause 6.4. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Syntax | Number of bits | Identifier |  |  |  |  |  |  |  |  |  |  |  |  |  |
| adaptation_field_data_descriptor() { | 8 | uimsbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| descriptor_tag | 8 | uimsbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| descriptor_length | 8 | bslbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| adaptation_field_data_identifier |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| } |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.1, PDF pages 3-3. 1 tables / 10 rows reproduced verbatim._
