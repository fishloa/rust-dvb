# C2 Delivery System (extension sub-tag 0x0D)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.6.1
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x0D-c2_delivery_system.rs`
**Rust struct:** `C2DeliverySystemDescriptor<'a>`

## Tables

### Table 114 — CP identifier descriptor
_PDF pages 117-117 (§6.4.6.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| CP_identifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 16 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| CP_system_id |  |  |
| } |  |  |
| } |  |  |

### Table 115 — C2 delivery system descriptor
_PDF pages 117-117 (§6.4.6.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| C2_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| plp_id | 8 | uimsbf |
| data_slice_id | 32 | bslbf |
| C2_System_tuning_frequency | 2 | uimsbf |
| C2_System_tuning_frequency_type | 3 | bslbf |
| active_OFDM_symbol_duration | 3 | bslbf |
| guard_interval |  |  |
| } |  |  |

### Table 116 — C2 tuning frequency type coding
_PDF pages 118-118 (§6.4.6.1)_

| C2_System_tuning_frequency_type | Description |
|---|---|
| 0b00 | Data Slice tuning frequency |
|  | This is the default option for DVB-C2 systems. The |
|  | C2_System_tuning_frequency field conveys the tuning frequency of the data |
|  | slice to which plp_id refers. The C2_System_tuning_frequency for a particular |
|  | Data Slice is the sum of the L1 signalling parameters START_FREQUENCY |
|  | and the DSLICE_TUNE_POS. Note that the Data Slice tuning frequency |
|  | information in the Layer One (first or bottom-most layer) (L1) signalling as well |
|  | as in the C2_delivery_system_descriptor have to be updated |
|  | synchronously. |
| 0b01 | C2 system centre frequency |
|  | This option is used by DVB-C2 head-ends that are not able to update the Data |
|  | Slice tuning frequency information in the C2_delivery_system_descriptor |
|  | and the L1 signalling in a synchronous way. The |
|  | C2_System_tuning_frequency is the centre frequency of the DVB-C2 system, |
|  | and it is required that a complete Preamble can be received. The receiver |
|  | needs to evaluate the L1 signalling in the preamble to get knowledge of the |
|  | final tuning position. |
| 0b10 | Initial tuning position for a (dependent) Static Data Slice |
|  | Signalling of this option implies that the Data Slice to be demodulated is a |
|  | (dependent) Static Slice. In the case of tuning to a (dependent) Static Data |
|  | Slice, it cannot be guaranteed that the receiver is able to decode the L1 |
|  | signalling at its final tuning position. Therefore the receiver will first tune to the |
|  | signalled initial C2_System_tuning_frequency where a complete Preamble is |
|  | transmitted. This frequency will usually be the DVB-C2 system centre |
|  | frequency, but can be any tuning position where the receiver can reliably |
|  | decode the L1 signal. The receiver needs to evaluate the L1 signalling in the |
|  | preamble in order to determine additional parameters (particularly notch |
|  | positions) as well as the final tuning frequency of the (dependent) Static Data |
|  | Slice. |
| 0b11 | reserved for future use. |

### Table 117 — Active OFDM symbol duration coding
_PDF pages 118-118 (§6.4.6.1)_

| active_OFDM_symbol_duration | Description |
|---|---|
|  | μ |
| 0b000 | 448 s (4k Fast Fourier Transform (FFT) mode for 8 MHz |
|  | Cable Television (CATV) systems) |
|  | μ |
| 0b001 | 597,33 s (4k FFT mode for 6 MHz CATV systems) |
| 0b010 to 0b111 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.6.1, PDF pages 4-4. 4 tables / 13 rows reproduced verbatim._
