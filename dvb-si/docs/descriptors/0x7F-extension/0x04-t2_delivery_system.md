# T2 Delivery System (extension sub-tag 0x04)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.6.3
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x04-t2_delivery_system.rs`
**Rust struct:** `T2DeliverySystemDescriptor<'a>`

## Spec text

### §6.4.6.3 T2 delivery system descriptor ............................................................................................................. 123

## Tables

### Table 133 — T2 delivery system descriptor
_PDF pages 124-124 (§6.4.6.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| T2_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| plp_id | 16 | uimsbf |
| T2_system_id | 2 | bslbf |
| if (descriptor_length > 4) { | 4 | bslbf |
| SISO_MISO | 2 | bslbf |
| bandwidth | 3 | bslbf |
| reserved_future_use | 3 | bslbf |
| guard_interval | 1 | bslbf |
| transmission_mode | 1 | bslbf |
| other_frequency_flag | 16 | uimsbf |
| tfs_flag | 8 | uimsbf |
| for (i=0;i<N;i++) { | 32 | uimsbf |
| cell_id | 32 | uimsbf |
| if (tfs_flag == 0b1) { | 8 | uimsbf |
| frequency_loop_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 32 | uimsbf |
| centre_frequency |  |  |
| } |  |  |
| } else { |  |  |
| centre_frequency |  |  |
| } |  |  |
| subcell_info_loop_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| cell_id_extension |  |  |
| transposer_frequency |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 134 — SISO/MISO mode coding
_PDF pages 124-124 (§6.4.6.3)_

| SISO_MISO | Description |
|---|---|
| 0b00 | Single Input, Single Output (SISO) |
| 0b01 | Multiple Input, Single Output (MISO) |
| 0b10 to 0b11 | reserved for future use |

### Table 135 — Bandwidth coding
_PDF pages 125-125 (§6.4.6.3)_

| bandwidth | Description |
|---|---|
| 0b0000 | 8 MHz |
| 0b0001 | 7 MHz |
| 0b0010 | 6 MHz |
| 0b0011 | 5 MHz |
| 0b0100 | 10 MHz |
| 0b0101 | 1,712 MHz |
| 0b0110 to 0b1111 | reserved for future use |

### Table 136 — Guard interval coding
_PDF pages 125-125 (§6.4.6.3)_

| guard_interval | Description |
|---|---|
| 0b000 | 1/32 |
| 0b001 | 1/16 |
| 0b010 | 1/8 |
| 0b011 | 1/4 |
| 0b100 | 1/128 |
| 0b101 | 19/128 |
| 0b110 | 19/256 |
| 0b111 | reserved for future use |

### Table 137 — Transmission mode coding
_PDF pages 125-125 (§6.4.6.3)_

| transmission_mode | Description |
|---|---|
| 0b000 | 2k mode |
| 0b001 | 8k mode |
| 0b010 | 4k mode |
| 0b011 | 1k mode |
| 0b100 | 16k mode |
| 0b101 | 32k mode |
| 0b110 to 0b111 | reserved for future use |

### Table 138 — TFS flag coding
_PDF pages 125-125 (§6.4.6.3)_

| tfs_flag | Description |
|---|---|
| 0b0 | no TFS arrangement in place |
| 0b1 | TFS arrangement in place |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.6.3, PDF pages 5-24. 6 tables / 34 rows reproduced verbatim._
