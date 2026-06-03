# Message (extension sub-tag 0x08)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.8
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x08-message.rs`
**Rust struct:** `MessageDescriptor<'a>`

## Tables

### Table 146 — Icon transport mode coding
_PDF pages 136-136 (§6.4.8)_

| icon_transport_mode | Description |
|---|---|
| 0 | the icon is delivered in the icon_data_byte sequence of bytes |
| 1 | the location of the icon file is identified by the URL conveyed in |
|  | the url_char sequence of bytes |
| 2 to 3 | reserved for future use |

### Table 147 — Coordinate system coding
_PDF pages 136-136 (§6.4.8)_

| coordinate_system | Description |
|---|---|
| 0 | the coordinate system is 720 × 576 |
| 1 | the coordinate system is 1 280 × 720 |
| 2 | the coordinate system is 1 920 × 1 080 |
| 3 to 6 | reserved for future use |
| 7 | user defined |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.8, PDF pages 5-24. 2 tables / 10 rows reproduced verbatim._
