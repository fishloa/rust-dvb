# S2x Satellite Delivery System (extension sub-tag 0x17)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.6.5
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x17-s2x_satellite_delivery_system.rs`
**Rust struct:** `S2xSatelliteDeliverySystemDescriptor<'a>`

## Tables

### Table 140 — S2X satellite delivery system descriptor
_PDF pages 128-128 (§6.4.6.5.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| S2X_satellite_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 5 | bslbf |
| receiver_profiles | 3 | bslbf |
| reserved_zero_future_use | 2 | uimsbf |
| S2X_mode | 1 | bslbf |
| scrambling_sequence_selector | 3 | bslbf |
| reserved_zero_future_use | 2 | bslbf |
| TS_GS_S2X_mode | 6 | bslbf |
| if (scrambling_sequence_selector == 0b1) { | 18 | uimsbf |
| reserved_zero_future_use | 32 | bslbf |
| scrambling_sequence_index | 16 | bslbf |
| } | 1 | bslbf |
| frequency (see note) | 2 | bslbf |
| orbital_position (see note) | 1 | bslbf |
| west_east_flag (see note) | 1 | bslbf |
| polarization (see note) | 3 | bslbf |
| multiple_input_stream_flag (see note) | 4 | bslbf |
| reserved_zero_future_use | 28 | bslbf |
| roll_off (see note) | 8 | uimsbf |
| reserved_zero_future_use | 8 | uimsbf |
| symbol_rate (see note) | 7 | bslbf |
| if (multiple_input_stream_flag == 0b1) { | 1 | uimsbf |
| input_stream_identifier (see note) | 32 | bslbf |
| } | 16 | bslbf |
| if (S2X_mode == 2) { | 1 | bslbf |
| timeslice_number | 2 | bslbf |
| } | 1 | bslbf |
| if (S2X_mode == 3) { | 1 | bslbf |
| reserved_zero_future_use | 3 | bslbf |
| num_channel_bonds_minus_one | 4 | bslbf |
| for (i=0;i<N;i++) { | 28 | bslbf |
| frequency | 8 | uimsbf |
| orbital_position | 8 | bslbf |
| west_east_flag |  |  |
| polarization |  |  |
| bonded_channel_multiple_input_stream_flag |  |  |
| reserved_zero_future_use |  |  |
| roll_off |  |  |
| reserved_zero_future_use |  |  |
| symbol_rate |  |  |
| if (bonded_channel_multiple_input_stream_flag == 0b1) { |  |  |
| input_stream_identifier |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| reserved_future_use |  |  |
| } |  |  |
| } |  |  |
| NOTE: When channel bonding is used, these parameters describe the primary channel. |  |  |

### Table 141 — Receiver profiles coding
_PDF pages 129-129 (§6.4.6.5.2)_

| receiver_profiles bit | Description |
|---|---|
| b(cid:0) (see note) | broadcast services |
| b(cid:2) | interactive services |
| b(cid:3) | DSNG services |
| b(cid:4) | professional services |
| b(cid:5) | Very Low Signal to Noise Ratio (VL-SNR) services |
| NOTE: This bit is transmitted last (see clause 5.1.6). |  |

### Table 142 — S2X mode coding
_PDF pages 129-129 (§6.4.6.5.2)_

| S2X_mode | Description |
|---|---|
| 0 | reserved for future use |
| 1 | S2X |
| 2 | S2X + time slicing |
| 3 | S2X + channel bonding |

### Table 143 — TS/GS S2X mode coding
_PDF pages 129-129 (§6.4.6.5.2)_

| TS_GS_S2X_mode (see note) | Description |
|---|---|
| 0 | generic packetized |
| 1 | GSE |
| 2 | GSE high efficiency mode |
| 3 | DVB transport stream |
| NOTE: These values are compatible with the coding of the TS/GS field in the |  |
| BBFrame header of DVB-S2X (see clause 5.1.6 of ETSI EN 302 307-2 [8]). |  |

### Table 144 — S2X roll off coding
_PDF pages 129-129 (§6.4.6.5.2)_

| roll_off | Description |
|---|---|
|  | α |
| 0b000 | = 0,35 |
|  | α |
| 0b001 | = 0,25 |
|  | α |
| 0b010 | = 0,20 |
| 0b011 | reserved for future use |
|  | α |
| 0b100 | = 0,15 |
|  | α |
| 0b101 | = 0,10 |
|  | α |
| 0b110 | = 0,05 |
| 0b111 | reserved for future use |

### Table 144a — S2Xv2 satellite delivery system descriptor
_PDF pages 130-130 (§6.4.6.5.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| S2Xv2_satellite_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension |  |  |
| S2Xv2_satellite_delivery_system_info() |  |  |
| } |  |  |

### Table 144b — S2Xv2 satellite delivery system info
_PDF pages 131-131 (§6.4.6.5.3)_

| Syntax | Number | Identifier |
|---|---|---|
|  | of bits |  |
| S2Xv2_satellite_delivery_system_info() { | 32 | uimsbf |
| delivery_system_id | 4 | uimsbf |
| S2Xv2_mode | 1 | bslbf |
| multiple_input_stream_flag | 3 | bslbf |
| roll_off | 2 | bslbf |
| reserved_zero_future_use | 1 | bslbf |
| NCR_reference | 1 | bslbf |
| NCR_version | 2 | uimsbf |
| channel_bond | 2 | bsblf |
| polarization | 1 | bslbf |
| if (S2Xv2_mode == 1 or S2Xv2_mode == 2) { | 1 | bslbf |
| scrambling_sequence_selector | 2 | bslbf |
| } else { | 5 | bslbf |
| reserved_zero_future_use | 24 | uimsbf |
| } | 32 | bslbf |
| TS_GS_S2X_mode | 32 | bslbf |
| receiver_profiles | 8 | uimsbf |
| satellite_id | 6 | bslbf |
| frequency | 18 | uimsbf |
| symbol_rate | 8 | uimsbf |
| if (multiple_input_stream_flag == 1) { | 7 | bslbf |
| input_stream_identifier | 1 | uimsbf |
| } | 32 | uimsbf |
| if (S2Xv2_mode == 1 or S2Xv2_mode == 2) { | 8 | uimsbf |
| if (scrambling_sequence_selector == 1) { | 1 | bsblf |
| reserved_zero_future_use | 1 | bsblf |
| scrambling_sequence_index | 2 | bsblf |
| } | 20 | uimsbf |
| } | 4 | bslf |
| if (S2Xv2_mode == 2 or S2Xv2_mode == 5) { | 4 | bsblf |
| timeslice_number | 20 | uimsbf |
| } | 32 | uimsbf |
| if (channel_bond == 1) { | 5 | uimsbf |
| reserved_zero_future_use | 3 | bslf |
| num_channel_bonds_minus_one | 8 | bslbf |
| for (i=0;i<N;i++) { |  |  |
| secondary_delivery_system_id |  |  |
| } |  |  |
| } |  |  |
| if (S2Xv2_mode == 4 or S2Xv2_mode == 5) { |  |  |
| SOSF_WH_sequence_number |  |  |
| SFFI_selector |  |  |
| beam_hopping_time_plan_selector |  |  |
| reserved_zero_future_use |  |  |
| reference_scrambing_index |  |  |
| if (SFFI_selector == 1) { |  |  |
| SFFI |  |  |
| } else { |  |  |
| reserved_zero_future_use |  |  |
| } |  |  |
| payload_scrambling_index |  |  |
| if (beam_hopping_time_plan_selector == 1) { |  |  |
| beamhopping_time_plan_id |  |  |
| } |  |  |
| superframe_pilots_WH_sequence_number |  |  |
| postamble_PLI |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| reserved_zero_future_use |  |  |
| } |  |  |
| } |  |  |

### Table 144c — S2Xv2 mode coding
_PDF pages 132-132 (§6.4.6.5.3)_

| S2Xv2_mode | Description |
|---|---|
| 0 | reserved for future use |
| 1 | S2X |
| 2 | S2X + time slicing |
| 3 | reserved for future use |
| 4 | S2X superframe (Annex E of ETSI EN 302 307-2 [8]) |
| 5 | S2X superframe (Annex E of ETSI EN 302 307-2 [8]) + |
|  | timeslicing (Annex M of ETSI EN 302 307-1 [7]) |
| 6-15 | reserved for future use |

### Table 144d — NCR version coding
_PDF pages 132-132 (§6.4.6.5.3)_

| NCR_reference | Description |
|---|---|
| 0 | The first symbol of the Start Of Frame field of the relevant DVB-S2 or DVB-S2X |
|  | physical layer frame |
| 1 | The first symbol of the Start Of Superframe (SOSF) field of the ETSI |
|  | EN 302 307-2 [8], annex E superframe |
| NCR_version | Description |
| 0 | NCR |
| 1 | NCR_v2 |

### Table 144e — channel bond coding
_PDF pages 133-133 (§6.4.6.5.3)_

| channel_bond | Description |
|---|---|
| 0 | Not part of a channel bond |
| 1 | Channel bond primary |
| 2 | Channel bond secondary |
| 3 | Reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.6.5, PDF pages 5-24. 10 tables / 53 rows reproduced verbatim._
