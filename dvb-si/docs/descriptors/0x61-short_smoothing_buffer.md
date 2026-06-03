# Short Smoothing Buffer (tag 0x61)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.38
**Parser file:** `crates/dvb_si/src/descriptors/0x61-short_smoothing_buffer.rs`
**Rust struct:** `ShortSmoothingBufferDescriptor<'a>`

## Tables

### Table 94 — Short smoothing buffer descriptor
_PDF pages 103-103 (§6.2.38)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| short_smoothing_buffer_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 2 | uimsbf |
| sb_size | 6 | uimsbf |
| sb_leak_rate | 8 | bslbf |
| for (i=0;i<N;i++) { |  |  |
| reserved_future_use |  |  |
| } |  |  |
| } |  |  |

### Table 95 — Smoothing buffer size coding
_PDF pages 103-103 (§6.2.38)_

| sb_size | Buffer size (bytes) |
|---|---|
| 0 | reserved for future use |
| 1 | 1 536 |
| 2 | reserved for future use |
| 3 | reserved for future use |

### Table 96 — Smoothing buffer leak rate coding
_PDF pages 104-104 (§6.2.38)_

| sb_leak_rate | Leak rate (Mbit/s) |
|---|---|
| 0 | reserved for future use |
| 1 | 0,0009 |
| 2 | 0,0018 |
| 3 | 0,0036 |
| 4 | 0,0072 |
| 5 | 0,0108 |
| 6 | 0,0144 |
| 7 | 0,0216 |
| 8 | 0,0288 |
| 9 | 0,075 |
| 10 | 0,5 |
| 11 | 0,5625 |
| 12 | 0,8437 |
| 13 | 1,0 |
| 14 | 1,1250 |
| 15 | 1,5 |
| 16 | 1,6875 |
| 17 | 2,0 |
| 18 | 2,2500 |
| 19 | 2,5 |
| 20 | 3,0 |
| 21 | 3,3750 |
| 22 | 3,5 |
| 23 | 4,0 |
| 24 | 4,5 |
| 25 | 5,0 |
| 26 | 5,5 |
| 27 | 6,0 |
| 28 | 6,5 |
| 29 | 6,7500 |
| 30 | 7,0 |
| 31 | 7,5 |
| 32 | 8,0 |
| 33 | 9,0 |
| 34 | 10,0 |
| 35 | 11,0 |
| 36 | 12,0 |
| 37 | 13,0 |
| 38 | 13,5 |
| 39 | 14,0 |
| 40 | 15,0 |
| 41 | 16,0 |
| 42 | 17,0 |
| 43 | 18,0 |
| 44 | 20,0 |
| 45 | 22,0 |
| 46 | 24,0 |
| 47 | 26,0 |
| 48 | 27,0 |
| 49 | 28,0 |
| 50 | 30,0 |
| 51 | 32,0 |
| 52 | 34,0 |
| 53 | 36,0 |
| 54 | 38,0 |
| 55 | 40,0 |
| 56 | 44,0 |
| 57 | 48,0 |
| 58 | 54,0 |
| 59 | 72,0 |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.38, PDF pages 4-4. 3 tables / 68 rows reproduced verbatim._
