# Linkage Descriptor (tag 0x4A)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.19
**Parser file:** `crates/dvb_si/src/descriptors/linkage.rs`
**Rust struct:** `LinkageDescriptor<'a>`

## Tables

### Table 59 — Linkage descriptor
_PDF pages 84-84 (§6.2.19.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| linkage_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| original_network_id | 16 | uimsbf |
| service_id | 8 | uimsbf |
| linkage_type | 8 | bslbf |
| if (linkage_type == 0x08) { |  |  |
| mobile_hand-over_info() |  |  |
| } else if (linkage_type == 0x0D) { |  |  |
| event_linkage_info() |  |  |
| } else if (linkage_type >= 0x0E && linkage_type <= 0x1F) { |  |  |
| extended_event_linkage_info() |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| private_data_byte |  |  |
| } |  |  |
| } |  |  |

### Table 60 — Linkage type coding
_PDF pages 84-84 (§6.2.19.1)_

| linkage_type | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | information service |
| 0x02 | EPG service |
| 0x03 | CA replacement service |
| 0x04 | TS containing complete network/bouquet SI |
| 0x05 | service replacement service |
| 0x06 | data broadcast service |
| 0x07 | Return Channel Satellite (RCS) map |
| 0x08 | mobile hand-over |
| 0x09 | System Software Update (SSU) service (ETSI TS 102 006 [20]) |
| 0x0A | TS containing SSU BAT or NIT (ETSI TS 102 006 [20]) |
| 0x0B | Internet Protocol/Medium Access Control (IP/MAC) notification service |
|  | (ETSI EN 301 192 [3]) |
| 0x0C | TS containing INT BAT or NIT (ETSI EN 301 192 [3]) |
| 0x0D | event linkage (see note) |
| 0x0E to 0x1F | extended event linkage (see note) |
| 0x20 | downloadable font info linkage (ETSI EN 303 560 [12]) |
| 0x21 | Native IP bootstrap MPE stream (DVB BlueBook A180) [57] |
| 0x22 to 0x7F | reserved for future use |
| 0x80 to 0xFE | user defined |
| 0xFF | reserved for future use |
| NOTE: A linkage_type with a value in the range 0x0D to 0x1F is only valid when the |  |
| descriptor is carried in the EIT. |  |

### Table 61 — Mobile hand-over info
_PDF pages 85-85 (§6.2.19.2)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| mobile_hand-over_info() { | 4 | uimsbf |
| hand-over_type | 3 | bslbf |
| reserved_future_use | 1 | bslbf |
| origin_type | 16 | uimsbf |
| if (hand-over_type == 0x1 | 16 | uimsbf |
| \|\| hand-over_type == 0x2 |  |  |
| \|\| hand-over_type == 0x3) { |  |  |
| network_id |  |  |
| } |  |  |
| if (origin_type == 0b0) { |  |  |
| initial_service_id |  |  |
| } |  |  |
| } |  |  |

### Table 62 — Hand-over type coding
_PDF pages 85-85 (§6.2.19.2)_

| hand-over_type | Description |
|---|---|
| 0x0 | reserved for future use |
| 0x1 | DVB hand-over to an identical service in a neighbouring country |
| 0x2 | DVB hand-over to a local variation of the same service |
| 0x3 | DVB hand-over to an associated service |
| 0x4 to 0xF | reserved for future use |

### Table 63 — Origin type coding
_PDF pages 85-85 (§6.2.19.2)_

| origin_type | Description |
|---|---|
| 0b0 | NIT |
| 0b1 | SDT |

### Table 64 — Event linkage info
_PDF pages 86-86 (§6.2.19.4)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| event_linkage_info() { | 16 | uimsbf |
| target_event_id | 1 | bslbf |
| target_listed | 1 | bslbf |
| event_simulcast | 6 | bslbf |
| reserved_future_use |  |  |
| } |  |  |

### Table 65 — Extended event linkage info
_PDF pages 87-87 (§6.2.19.4)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| extended_event_linkage_info() { | 8 | uimsbf |
| loop_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| target_event_id | 1 | bslbf |
| target_listed | 2 | uimsbf |
| event_simulcast | 2 | uimsbf |
| link_type | 1 | bslbf |
| target_id_type | 1 | bslbf |
| original_network_id_flag | 16 | uimsbf |
| service_id_flag | 16 | uimsbf |
| if (target_id_type == 3) { | 16 | uimsbf |
| user_defined_id | 16 | uimsbf |
| } else { |  |  |
| if (target_id_type == 1) { |  |  |
| target_transport_stream_id |  |  |
| } |  |  |
| if (original_network_id_flag == 0b1) { |  |  |
| target_original_network_id |  |  |
| } |  |  |
| if (service_id_flag == 0b1) { |  |  |
| target_service_id |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 66 — Link type coding
_PDF pages 88-88 (§6.2.19.4)_

| link_type | linkage_type (see note) | Type of target service |
|---|---|---|
| 0 | 0x0E | Standard Definition (SD) |
| 1 | 0x0E | High Definition (HD) |
| 2 | 0x0E | frame compatible plano-stereoscopic H.264/AVC |
| 3 | 0x0E | service compatible plano-stereoscopic Multi-View video Coding (MVC) |
| 0 | 0x0F | Ultra High Definition (UHD) |
| 1 | 0x0F | service frame compatible plano-stereoscopic |
| 2 to 3 | 0x0F | reserved for future use |
| 0 to 3 | 0x10 to 0x1F | reserved for future use |
| NOTE: See table 60. |  |  |

### Table 67 — Target id type coding
_PDF pages 88-88 (§6.2.19.4)_

| target_id_type | How target service is matched |
|---|---|
| 0 | use transport_stream_id |
| 1 | use target_transport_stream_id |
| 2 | match any transport_stream_id (wildcard) |
| 3 | use user_defined_id |

### Table 68 — Target service matching rules
_PDF pages 88-88 (§6.2.19.4)_

| epyt_di_tegrat | galf_di_krowten_lanigiro | galf_di_ecivres | ? |  | ? |  | ? |  | maerts_tropsnart_tegrat |  | _krowten_lanigiro_tegrat |  | ? |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  | di_maerts_tropsnart |  | di_krowten_lanigiro |  | di_ecivres |  | hctam ? |  | hctam |  | di_ecivres_tegrat |  |
|  |  |  | hctam |  | hctam |  | hctam |  | di_ |  | ? |  | hctam |  |
|  |  |  |  |  |  |  |  |  |  |  | di |  |  |  |
| 0 | 0b0 | 0b0 | ✓ |  | ✓ |  | ✓ |  |  |  |  |  |  |  |
| 0 | 0b0 | 0b1 | ✓ |  | ✓ |  |  |  |  |  |  |  | ✓ |  |
| 0 | 0b1 | 0b0 | ✓ |  |  |  | ✓ |  |  |  | ✓ |  |  |  |
| 0 | 0b1 | 0b1 | ✓ |  |  |  |  |  |  |  | ✓ |  | ✓ |  |
| 1 | 0b0 | 0b0 |  |  | ✓ |  | ✓ |  | ✓ |  |  |  |  |  |
| 1 | 0b0 | 0b1 |  |  | ✓ |  |  |  | ✓ |  |  |  | ✓ |  |
| 1 | 0b1 | 0b0 |  |  |  |  | ✓ |  | ✓ |  | ✓ |  |  |  |
| 1 | 0b1 | 0b1 |  |  |  |  |  |  | ✓ |  | ✓ |  | ✓ |  |
| 2 (see note) | 0b0 | 0b0 |  |  | ✓ |  | ✓ |  |  |  |  |  |  |  |
| 2 (see note) | 0b0 | 0b1 |  |  | ✓ |  |  |  |  |  |  |  | ✓ |  |
| 2 (see note) | 0b1 | 0b0 |  |  |  |  | ✓ |  |  |  | ✓ |  |  |  |
| 2 (see note) | 0b1 | 0b1 |  |  |  |  |  |  |  |  | ✓ |  | ✓ |  |
| 3 | n/a | n/a | All services matched with user_defined_id |  |  |  |  |  |  |  |  |  |  |  |
| NOTE: When target_id_type is set to two, neither transport_stream_id, nor |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| target_transport_stream_id are used for matching. Instead, all services with matching |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| remaining identifiers as shown, are considered matches. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.19, PDF pages 4-4. 10 tables / 69 rows reproduced verbatim._
