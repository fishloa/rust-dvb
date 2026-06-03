# Announcement Support (tag 0x6E)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.3
**Parser file:** `crates/dvb_si/src/descriptors/0x6E-announcement_support.rs`
**Rust struct:** `AnnouncementSupportDescriptor<'a>`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 14 — Adaptation field data identifier coding
_PDF pages 55-55 (§6.2.3)_

| adaptation_field_data_identifier bit | Description |
|---|---|
| b(cid:0) (see note) | announcement_switching_data |
| b(cid:2) | AU_information |
| b(cid:3) | PVR_assist_information |
| b(cid:4) | reserved_zero_future_use |
| b(cid:5) | reserved_zero_future_use |
| b(cid:6) | reserved_zero_future_use |
| b(cid:7) | reserved_zero_future_use |
| b(cid:8) | reserved_zero_future_use |
| NOTE: This bit is transmitted last (see clause 5.1.6). |  |

### Table 15 — Ancillary data descriptor
_PDF pages 55-55 (§6.2.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| ancillary_data_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | bslbf |
| ancillary_data_identifier |  |  |
| } |  |  |

### Table 16 — Ancillary data identifier coding
_PDF pages 55-55 (§6.2.3)_

| ancillary_data_identifier | Description |
|---|---|
| bit |  |
| b(cid:0) (see note) | Digital Versatile Disc (DVD) Video Ancillary Data (ETSI TS 101 154 [14]) |
| b(cid:2) | Extended Ancillary Data (ETSI TS 101 154 [14]) |
| b(cid:3) | Announcement Switching Data (ETSI TS 101 154 [14]) |
| b(cid:4) | Digital Audio Broadcasting (DAB) Ancillary Data (ETSI EN 300 401 [29]) |
| b(cid:5) | Scale Factor Error Check (ScF-CRC) (ETSI TS 101 154 [14]) |
| b(cid:6) | MPEG-4 ancillary data (ETSI TS 101 154 [14], clause C.5) |
| b(cid:7) | Radio Data System (RDS) via Universal Encoder Communication Protocol |
|  | (UECP) (ETSI TS 101 154 [14]) |
| b(cid:8) | reserved_zero_future_use |
| NOTE: This bit is transmitted last (see clause 5.1.6). |  |

### Table 17 — Announcement support descriptor
_PDF pages 56-56 (§6.2.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| announcement_support_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | bslbf |
| announcement_support_indicator | 4 | uimsbf |
| for (i=0;i<N;i++) { | 1 | bslbf |
| announcement_type | 3 | uimsbf |
| reserved_future_use | 16 | uimsbf |
| reference_type | 16 | uimsbf |
| if (reference_type == 0x01 | 16 | uimsbf |
| \|\| reference_type == 0x02 | 8 | uimsbf |
| \|\| reference_type == 0x03) { |  |  |
| original_network_id |  |  |
| transport_stream_id |  |  |
| service_id |  |  |
| component_tag |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 18 — Void
_PDF pages 56-56 (§6.2.3)_

| announcement_type | Description |
|---|---|
| 0b0000 | Emergency alarm |
| 0b0001 | Road Traffic flash |
| 0b0010 | Public Transport flash |
| 0b0011 | Warning message |
| 0b0100 | News flash |
| 0b0101 | Weather flash |
| 0b0110 | Event announcement |
| 0b0111 | Personal call |
| 0b1000 to 0b1111 | Reserved for future use |

### Table 19 — Announcement type coding
_PDF pages 56-56 (§6.2.3)_

| reference_type | Description |
|---|---|
| 0b000 | Announcement is broadcast in the usual audio stream of the service |
| 0b001 | Announcement is broadcast in a separate audio stream that is part of the service |
| 0b010 | Announcement is broadcast by means of a different service within the same DVB transport stream |
| 0b011 | Announcement is broadcast by means of a different service within a different DVB transport stream |
| 0b100 to 0b111 | Reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.3, PDF pages 3-3. 6 tables / 40 rows reproduced verbatim._
