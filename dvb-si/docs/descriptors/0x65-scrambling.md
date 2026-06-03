# Scrambling (tag 0x65)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.32
**Parser file:** `crates/dvb_si/src/descriptors/0x65-scrambling.rs`
**Rust struct:** `ScramblingDescriptor<'a>`

## Tables

### Table 85 — Private data specifier descriptor
_PDF pages 98-98 (§6.2.32)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| private_data_specifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | uimsbf |
| private_data_specifier |  |  |
| } |  |  |

### Table 86 — Scrambling descriptor
_PDF pages 98-98 (§6.2.32)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| scrambling_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| scrambling_mode |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.32, PDF pages 4-4. 2 tables / 4 rows reproduced verbatim._
