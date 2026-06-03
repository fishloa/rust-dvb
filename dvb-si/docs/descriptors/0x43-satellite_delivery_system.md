# Satellite Delivery System Descriptor (tag 0x43)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.13.2
**Parser file:** `crates/dvb_si/src/descriptors/satellite_delivery_system.rs`
**Rust struct:** `SatelliteDeliverySystemDescriptor`

## Tables

### Table 35 — Modulation scheme for cable
_PDF pages 73-73 (§6.2.13.2)_

| modulation | Description |
|---|---|
| 0x00 | not defined |
| 0x01 | 16-ary Quadrature Amplitude Modulation (16QAM) |
| 0x02 | 32-ary Quadrature Amplitude Modulation (32QAM) |
| 0x03 | 64-ary Quadrature Amplitude Modulation (64QAM) |
| 0x04 | 128-ary Quadrature Amplitude Modulation (128QAM) |
| 0x05 | 256-ary Quadrature Amplitude Modulation (256QAM) |
| 0x06 to 0xFF | reserved for future use |

### Table 36 — Inner FEC scheme
_PDF pages 73-73 (§6.2.13.2)_

| FEC_inner (see note) | Description |
|---|---|
| 0b0000 | not defined |
| 0b0001 | 1/2 convolutional code rate |
| 0b0010 | 2/3 convolutional code rate |
| 0b0011 | 3/4 convolutional code rate |
| 0b0100 | 5/6 convolutional code rate |
| 0b0101 | 7/8 convolutional code rate |
| 0b0110 | 8/9 convolutional code rate |
| 0b0111 | 3/5 convolutional code rate |
| 0b1000 | 4/5 convolutional code rate |
| 0b1001 | 9/10 convolutional code rate |
| 0b1010 to 0b1110 | reserved for future use |
| 0b1111 | no convolutional coding |
| NOTE: Not all convolutional code rates apply for all modulation schemes. |  |

### Table 37 — Satellite delivery system descriptor
_PDF pages 73-73 (§6.2.13.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| satellite_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | bslbf |
| frequency | 16 | bslbf |
| orbital_position | 1 | bslbf |
| west_east_flag | 2 | bslbf |
| polarization | 2 | bslbf |
| if (modulation_system == 0b1) { | 2 | bslbf |
| roll_off | 1 | bslbf |
| } else { | 2 | bslbf |
| reserved_zero_future_use | 28 | bslbf |
| } | 4 | bslbf |
| modulation_system |  |  |
| modulation_type |  |  |
| symbol_rate |  |  |
| FEC_inner |  |  |
| } |  |  |

### Table 38 — Polarization coding
_PDF pages 74-74 (§6.2.13.2)_

| polarization | Description |
|---|---|
| 0b00 | linear - horizontal |
| 0b01 | linear - vertical |
| 0b10 | circular - left |
| 0b11 | circular - right |

### Table 39 — Roll-off factor
_PDF pages 74-74 (§6.2.13.2)_

| roll_off | Description |
|---|---|
|  | α |
| 0b00 | = 0,35 |
|  | α |
| 0b01 | = 0,25 |
|  | α |
| 0b10 | = 0,20 |
| 0b11 | reserved for future use |

### Table 40 — Modulation system for satellite
_PDF pages 74-74 (§6.2.13.2)_

| modulation_system | Description |
|---|---|
| 0b0 | DVB-S |
| 0b1 | DVB-S2 |

### Table 41 — Modulation type for satellite
_PDF pages 74-74 (§6.2.13.2)_

| modulation_type | Description |
|---|---|
| 0b00 | auto |
| 0b01 | Quaternary Phase Shift Keying (QPSK) |
| 0b10 | 8-ary Phase Shift Keying (8PSK) |
| 0b11 | 16QAM (n/a for DVB-S2) |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.13.2, PDF pages 4-4. 7 tables / 42 rows reproduced verbatim._
