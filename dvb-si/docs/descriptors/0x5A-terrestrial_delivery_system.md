# Terrestrial Delivery System (tag 0x5A)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.13.4
**Parser file:** `crates/dvb_si/src/descriptors/0x5A-terrestrial_delivery_system.rs`
**Rust struct:** `TerrestrialDeliverySystemDescriptor<'a>`

## Tables

### Table 44 — Terrestrial delivery system descriptor
_PDF pages 76-76 (§6.2.13.4)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| terrestrial_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | uimsbf |
| centre_frequency | 3 | bslbf |
| bandwidth | 1 | bslbf |
| priority | 1 | bslbf |
| time_slicing_indicator | 1 | bslbf |
| MPE-FEC_indicator | 2 | bslbf |
| reserved_future_use | 2 | bslbf |
| constellation | 3 | bslbf |
| hierarchy_information | 3 | bslbf |
| code_rate_HP_stream | 3 | bslbf |
| code_rate_LP_stream | 2 | bslbf |
| guard_interval | 2 | bslbf |
| transmission_mode | 1 | bslbf |
| other_frequency_flag | 32 | bslbf |
| reserved_future_use |  |  |
| } |  |  |

### Table 45 — Bandwidth coding
_PDF pages 76-76 (§6.2.13.4)_

| bandwidth | Description |
|---|---|
| 0b000 | 8 MHz |
| 0b001 | 7 MHz |
| 0b010 | 6 MHz |
| 0b011 | 5 MHz |
| 0b100 to 0b111 | reserved for future use |

### Table 46 — Priority coding
_PDF pages 76-76 (§6.2.13.4)_

| priority | Description |
|---|---|
| 0b0 | HP |
| 0b1 | LP |

### Table 47 — Constellation coding
_PDF pages 77-77 (§6.2.13.4)_

| constellation | Description |
|---|---|
| 0b00 | QPSK |
| 0b01 | 16QAM |
| 0b10 | 64QAM |
| 0b11 | reserved for future use |

### Table 48 — Hierarchy information coding
_PDF pages 77-77 (§6.2.13.4)_

| hierarchy_information | Description |
|---|---|
| 0b000 | non-hierarchical, native interleaver |
|  | α |
| 0b001 | = 1, native interleaver |
|  | α |
| 0b010 | = 2, native interleaver |
|  | α |
| 0b011 | = 4, native interleaver |
| 0b100 | non-hierarchical, in-depth interleaver |
|  | α |
| 0b101 | = 1, in-depth interleaver |
|  | α |
| 0b110 | = 2, in-depth interleaver |
|  | α |
| 0b111 | = 4, in-depth interleaver |

### Table 49 — HP and LP stream code rate coding
_PDF pages 77-77 (§6.2.13.4)_

| code_rate_HP_stream and code_rate_LP_stream | Description |
|---|---|
|  | 1/2 |
| 0b000 | 2/3 |
| 0b001 | 3/4 |
| 0b010 | 5/6 |
| 0b011 | 7/8 |
| 0b100 |  |
| 0b101 to 0b111 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.13.4, PDF pages 4-4. 6 tables / 32 rows reproduced verbatim._
