# Sh Delivery System (extension sub-tag 0x05)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.6.2
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x05-sh_delivery_system.rs`
**Rust struct:** `ShDeliverySystemDescriptor<'a>`

## Tables

### Table 118 — C2 guard interval coding
_PDF pages 119-119 (§6.4.6.2)_

| guard_interval | Description |
|---|---|
|  | 1/128 |
| 0b000 | 1/64 |
| 0b001 |  |
| 0b0010 to 0b111 | reserved for future use |

### Table 119 — SH delivery system descriptor
_PDF pages 119-119 (§6.4.6.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| SH_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 4 | bslbf |
| diversity_mode | 4 | bslbf |
| reserved_future_use | 1 | bslbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| modulation_type | 1 | bslbf |
| interleaver_presence | 5 | bslbf |
| interleaver_type | 2 | bslbf |
| reserved_future_use | 2 | bslbf |
| if (modulation_type == 0b0) { | 2 | bslbf |
| polarization | 4 | bslbf |
| roll_off | 5 | bslbf |
| modulation_mode | 1 | bslbf |
| code_rate | 3 | bslbf |
| symbol_rate | 1 | bslbf |
| reserved_future_use | 3 | bslbf |
| } else { | 4 | bslbf |
| bandwidth | 2 | bslbf |
| priority | 2 | bslbf |
| constellation_and_hierarchy | 1 | bslbf |
| code_rate | 6 | uimsbf |
| guard_interval | 6 | uimsbf |
| transmission_mode | 6 | uimsbf |
| common_frequency | 8 | uimsbf |
| } | 6 | uimsbf |
| if (interleaver_presence == 0b1) { | 6 | uimsbf |
| if (interleaver_type == 0b0) { | 2 | uimsbf |
| common_multiplier |  |  |
| nof_late_taps |  |  |
| nof_slices |  |  |
| slice_distance |  |  |
| non_late_increments |  |  |
| } else { |  |  |
| common_multiplier |  |  |
| reserved_future_use |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 120 — Diversity mode coding
_PDF pages 120-120 (§6.4.6.2)_

| diversity_mode | paTS | FEC diversity | FEC at PHY | FEC at link |
|---|---|---|---|---|
| 0b0000 | no | no | no | no |
| 0b0001 to 0b0111 | reserved for future use |  |  |  |
| 0b1000 | yes | no | no | no |
| 0b1001 to 0b1100 | reserved for future use |  |  |  |
| 0b1101 | yes | yes | no | yes |
| 0b1110 | yes | yes | yes | no |
| 0b1111 | yes | yes | yes | yes |

### Table 121 — SH modulation type coding
_PDF pages 120-120 (§6.4.6.2)_

| modulation_type | Description |
|---|---|
| 0b0 | Time-Domain Multiplex (TDM) |
| 0b1 | OFDM |

### Table 122 — Interleaver presence coding
_PDF pages 120-120 (§6.4.6.2)_

| interleaver_presence | Description |
|---|---|
| 0b0 | no interleaver info follows |
| 0b1 | an interleaver info follows |

### Table 123 — Polarization coding
_PDF pages 120-120 (§6.4.6.2)_

| polarization | Description |
|---|---|
| 0b00 | linear - horizontal |
| 0b01 | linear - vertical |
| 0b10 | circular - left |
| 0b11 | circular - right |

### Table 124 — Roll-off factor
_PDF pages 120-120 (§6.4.6.2)_

| roll_off | Description |
|---|---|
|  | α |
| 0b00 | = 0,35 |
|  | α |
| 0b01 | = 0,25 |
|  | α |
| 0b10 | = 0,15 |
| 0b11 | reserved for future use |

### Table 125 — Modulation mode coding
_PDF pages 121-121 (§6.4.6.2)_

| modulation_mode | Description |
|---|---|
| 0b00 | QPSK |
| 0b01 | 8PSK |
| 0b10 | 16-ary Amplitude and Phase Shift Keying (16APSK) |
| 0b11 | reserved for future use |

### Table 126 — Code rate coding
_PDF pages 121-121 (§6.4.6.2)_

| code_rate | Description |
|---|---|
| 0b0000 | 1/5 standard |
| 0b0001 | 2/9 standard |
| 0b0010 | 1/4 standard |
| 0b0011 | 2/7 standard |
| 0b0100 | 1/3 standard |
| 0b0101 | 1/3 complementary |
| 0b0110 | 2/5 standard |
| 0b0111 | 2/5 complementary |
| 0b1000 | 1/2 standard |
| 0b1001 | 1/2 complementary |
| 0b1010 | 2/3 standard |
| 0b1011 | 2/3 complementary |
| 0b1100 to 0b1111 | reserved for future use |

### Table 127 — Symbol rate coding
_PDF pages 121-121 (§6.4.6.2)_

| symbol_rate | Equivalent | Equivalent guard | Symbol rate for | Symbol rate for | Symbol rate for |
|---|---|---|---|---|---|
|  | bandwidth | interval | α | α | α |
|  |  |  | = 0,15 | = 0,25 | = 0,35 |
| 0b0 0000 | 8 | 1/4 | 34/5 | 32/5 | 29/5 |
| 0b0 0001 | 8 | 1/8 | 62/9 | 56/9 | 52/9 |
| 0b0 0010 | 8 | 1/16 | 116/17 | 108/17 | 100/17 |
| 0b0 0011 | 8 | 1/32 | 224/33 | 208/33 | 64/11 |
| 0b0 0100 | 7 | 1/4 | 119/20 | 28/5 | 203/40 |
| 0b0 0101 | 7 | 1/8 | 217/36 | 49/9 | 91/18 |
| 0b0 0110 | 7 | 1/16 | 203/34 | 189/34 | 175/34 |
| 0b0 0111 | 7 | 1/32 | 196/33 | 182/33 | 56/11 |
| 0b0 1000 | 6 | 1/4 | 51/10 | 24/5 | 87/20 |
| 0b0 1001 | 6 | 1/8 | 31/6 | 14/3 | 13/3 |
| 0b0 1010 | 6 | 1/16 | 87/17 | 81/17 | 75/17 |
| 0b0 1011 | 6 | 1/32 | 56/11 | 52/11 | 48/11 |
| 0b0 1100 | 5 | 1/4 | 17/4 | 4/1 | 29/8 |
| 0b0 1101 | 5 | 1/8 | 155/36 | 35/9 | 65/18 |
| 0b0 1110 | 5 | 1/16 | 145/34 | 135/34 | 125/34 |
| 0b0 1111 | 5 | 1/32 | 140/33 | 130/33 | 40/11 |
| 0b1 0000 | 1,7 | 1/4 | 34/25 | 32/25 | 29/25 |
| 0b1 0001 | 1,7 | 1/8 | 62/45 | 56/45 | 52/45 |
| 0b1 0010 | 1,7 | 1/16 | 116/85 | 108/85 | 20/17 |
| 0b1 0011 | 1,7 | 1/32 | 224/165 | 208/165 | 64/55 |
| 0b1 0100 to 0b1 | reserved for future use |  |  |  |  |
| 1111 |  |  |  |  |  |

### Table 128 — Bandwidth coding
_PDF pages 122-122 (§6.4.6.2)_

| bandwidth | Description |
|---|---|
| 0b000 | 8 MHz |
| 0b001 | 7 MHz |
| 0b010 | 6 MHz |
| 0b011 | 5 MHz |
| 0b100 | 1,7 MHz |
| 0b101 to 0b111 | reserved for future use |

### Table 129 — Priority coding
_PDF pages 122-122 (§6.4.6.2)_

| constellation_and_hierarchy | priority | Description |
|---|---|---|
| 0b000 to 0b001 | 0b0 | n/a |
|  | 0b1 | no priority mode |
| 0b010 to 0b100 | 0b0 | LP |
|  | 0b1 | HP |

### Table 130 — Constellation and hierarchy coding
_PDF pages 122-122 (§6.4.6.2)_

| constellation_and_hierarchy | Description |
|---|---|
| 0b000 | QPSK |
| 0b001 | 16QAM, non-hierarchical |
|  | α |
| 0b010 | 16QAM, hierarchical, = 1 |
|  | α |
| 0b011 | 16QAM, hierarchical, = 2 |
|  | α |
| 0b100 | 16QAM, hierarchical, = 3 |
| 0b101 to 0b111 | reserved for future use |

### Table 131 — Guard interval coding
_PDF pages 122-122 (§6.4.6.2)_

| guard_interval | Description |
|---|---|
| 0b00 | 1/32 |
| 0b01 | 1/16 |
| 0b10 | 1/8 |
| 0b11 | 1/4 |

### Table 132 — Transmission mode coding
_PDF pages 122-122 (§6.4.6.2)_

| transmission_mode | Description |
|---|---|
| 0b00 | 1k mode |
| 0b01 | 2k mode |
| 0b10 | 4k mode |
| 0b11 | 8k mode |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.6.2, PDF pages 4-4. 15 tables / 100 rows reproduced verbatim._
