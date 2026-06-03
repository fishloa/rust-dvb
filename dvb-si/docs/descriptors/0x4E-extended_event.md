# Extended Event (tag 0x4E)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.15
**Parser file:** `crates/dvb_si/src/descriptors/0x4E-extended_event.rs`
**Rust struct:** `ExtendedEventDescriptor<'a>`

## Tables

### Table 50 — Guard interval coding
_PDF pages 78-78 (§6.2.15)_

| guard_interval | Description |
|---|---|
|  | 1/32 |
| 0b00 | 1/16 |
| 0b01 | 1/8 |
| 0b10 | 1/4 |
| 0b11 |  |

### Table 51 — Transmission mode coding
_PDF pages 78-78 (§6.2.15)_

| transmission_mode | Description |
|---|---|
| 0b00 | 2k mode |
| 0b01 | 8k mode |
| 0b10 | 4k mode |
| 0b11 | reserved for future use |

### Table 52 — DSNG descriptor
_PDF pages 78-78 (§6.2.15)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| DSNG_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| byte |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.15, PDF pages 4-4. 3 tables / 12 rows reproduced verbatim._
