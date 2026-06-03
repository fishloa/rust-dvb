# Aac (tag 0x7C)

**Spec:** ETSI EN 300 468 v1.19.1 §H
**Parser file:** `crates/dvb_si/src/descriptors/0x7C-aac.rs`
**Rust struct:** `AacDescriptor<'a>`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 13 — Adaptation field data descriptor
_PDF pages 54-54 (§6.2.1)_

| Descriptor | Tag | NIT |  | BAT |  | SDT |  | EIT |  | TOT |  | PMT |  | SIT |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  | value |  |  |  |  |  |  |  |  |  |  |  |  | (see note 1) |  |
| AAC_descriptor (see annex H) | 0x7C | - |  | - |  | - |  | - |  | - |  | ✓ |  | - |  |
| XAIT_location_descriptor (ETSI TS 102 727 [i.2]) | 0x7D | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  |
| FTA_content_management_descriptor | 0x7E | ✓ |  | ✓ |  | ✓ |  | ✓ |  | - |  | - |  | - |  |
| extension_descriptor (see note 4) | 0x7F | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  | ✓ |  |
| user defined | 0x80 to |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | 0xFE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| reserved for future use | 0xFF |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 1: Only found in Partial Transport Streams. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 2: Only in the TSDT. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 3: May also be located in the CAT (ISO/IEC 13818-1 [1]) and IP/MAC Notification Table (INT) (ETSI |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| EN 301 192 [3]). |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| NOTE 4: See also clause 6.3 and clause 6.4. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Syntax | Number of bits | Identifier |  |  |  |  |  |  |  |  |  |  |  |  |  |
| adaptation_field_data_descriptor() { | 8 | uimsbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| descriptor_tag | 8 | uimsbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| descriptor_length | 8 | bslbf |  |  |  |  |  |  |  |  |  |  |  |  |  |
| adaptation_field_data_identifier |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| } |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

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

### Table 21 — Bouquet name descriptor
_PDF pages 57-57 (§6.2.6)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| bouquet_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 22 — CA identifier descriptor
_PDF pages 57-57 (§6.2.6)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| CA_identifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| CA_system_id |  |  |
| } |  |  |
| } |  |  |

### Table 23 — Cell frequency link descriptor
_PDF pages 58-58 (§6.2.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cell_frequency_link_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 32 | uimsbf |
| cell_id | 8 | uimsbf |
| frequency | 8 | uimsbf |
| subcell_info_loop_length | 32 | uimsbf |
| for (j=0;j<N;j++) { |  |  |
| cell_id_extension |  |  |
| transposer_frequency |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 24 — Cell list descriptor
_PDF pages 58-58 (§6.2.7)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cell_list_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| cell_id | 16 | uimsbf |
| cell_latitude | 12 | uimsbf |
| cell_longitude | 12 | uimsbf |
| cell_extent_of_latitude | 8 | uimsbf |
| cell_extent_of_longitude | 8 | uimsbf |
| subcell_info_loop_length | 16 | uimsbf |
| for (j=0;j<N;j++) { | 16 | uimsbf |
| cell_id_extension | 12 | uimsbf |
| subcell_latitude | 12 | uimsbf |
| subcell_longitude |  |  |
| subcell_extent_of_latitude |  |  |
| subcell_extent_of_longitude |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 25 — Component descriptor
_PDF pages 60-60 (§6.2.8)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| component_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | uimsbf |
| stream_content_ext | 4 | uimsbf |
| stream_content | 8 | uimsbf |
| component_type | 8 | uimsbf |
| component_tag | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 26 — stream_content, stream_content_ext, and component_type coding
_PDF pages 60-66 (§6.2.8)_

| stream_content | stream_content_ext | component_type | Description |
|---|---|---|---|
| 0x0 | 0x0 to 0xF | 0x00 to 0xFF | reserved for future use |
| 0x1 | n/a (see note 1) | 0x00 | reserved for future use |
|  |  | 0x01 | MPEG-2 video, 4:3 aspect ratio, 25 Hz (see note 2) |
|  |  | 0x02 | MPEG-2 video, 16:9 aspect ratio with pan vectors, 25 Hz |
|  |  |  | (see note 2) |
|  |  | 0x03 | MPEG-2 video, 16:9 aspect ratio without pan vectors, |
|  |  |  | 25 Hz (see note 2) |
|  |  | 0x04 | MPEG-2 video, > 16:9 aspect ratio, 25 Hz (see note 2) |
|  |  | 0x05 | MPEG-2 video, 4:3 aspect ratio, 30 Hz (see note 2) |
|  |  | 0x06 | MPEG-2 video, 16:9 aspect ratio with pan vectors, 30 Hz |
|  |  |  | (see note 2) |
|  |  | 0x07 | MPEG-2 video, 16:9 aspect ratio without pan vectors, |
|  |  |  | 30 Hz (see note 2) |
|  |  | 0x08 | MPEG-2 video, > 16:9 aspect ratio, 30 Hz (see note 2) |
|  |  | 0x09 | MPEG-2 high definition video, 4:3 aspect ratio, 25 Hz |
|  |  |  | (see note 2) |
|  |  | 0x0A | MPEG-2 high definition video, 16:9 aspect ratio with pan |
|  |  |  | vectors, 25 Hz (see note 2) |
|  |  | 0x0B | MPEG-2 high definition video, 16:9 aspect ratio without |
|  |  |  | pan vectors, 25 Hz (see note 2) |
|  |  | 0x0C | MPEG-2 high definition video, > 16:9 aspect ratio, 25 Hz |
|  |  |  | (see note 2) |
|  |  | 0x0D | MPEG-2 high definition video, 4:3 aspect ratio, 30 Hz |
|  |  |  | (see note 2) |
|  |  | 0x0E | MPEG-2 high definition video, 16:9 aspect ratio with pan |
|  |  |  | vectors, 30 Hz (see note 2) |
|  |  | 0x0F | MPEG-2 high definition video, 16:9 aspect ratio without |
|  |  |  | pan vectors, 30 Hz (see note 2) |
|  |  | 0x10 | MPEG-2 high definition video, > 16:9 aspect ratio, 30 Hz |
|  |  |  | (see note 2) |
|  |  | 0x11 to 0xAF | reserved for future use |
|  |  | 0xB0 to 0xFE | user defined |
|  |  | 0xFF | reserved for future use |
| 0x2 | n/a (see note 1) | 0x00 | reserved for future use |
|  |  | 0x01 | MPEG-1 Layer 2 audio, single mono channel |
|  |  | 0x02 | MPEG-1 Layer 2 audio, dual mono channel |
|  |  | 0x03 | MPEG-1 Layer 2 audio, stereo (2 channel) |
|  |  | 0x04 | MPEG-1 Layer 2 audio, multi-lingual, multi-channel |
|  |  | 0x05 | MPEG-1 Layer 2 audio, surround sound |
|  |  | 0x06 to 0x3F | reserved for future use |
|  |  | 0x40 | MPEG-1 Layer 2 audio description for the visually |
|  |  |  | impaired (see note 3) |
|  |  | 0x41 | MPEG-1 Layer 2 audio for the hard of hearing |
|  |  | 0x42 | receiver-mix supplementary audio as per annex E of |
|  |  |  | ETSI TS 101 154 [14] |
|  |  | 0x43 to 0x46 | reserved for future use |
|  |  | 0x47 | MPEG-1 Layer 2 audio, receiver-mix audio description |
|  |  | 0x48 | MPEG-1 Layer 2 audio, broadcast-mix audio description |
|  |  | 0x49 to 0xAF | reserved for future use |
|  |  | 0xB0 to 0xFE | user defined |
|  |  | 0xFF | reserved for future use |
| 0x3 | n/a (see note 1) | 0x00 | reserved for future use |
|  |  | 0x01 | EBU Teletext subtitles |
|  |  | 0x02 | associated EBU Teletext |
|  |  | 0x03 | Vertical Blanking Interval (VBI) data |
|  |  | 0x04 to 0x0F | reserved for future use |
|  |  | 0x10 | DVB subtitles ETSI EN 300 743 [2] (normal) with no |
|  |  |  | monitor aspect ratio criticality |
|  |  | 0x11 | DVB subtitles ETSI EN 300 743 [2] (normal) for display |
|  |  |  | on 4:3 aspect ratio monitor |
|  |  | 0x12 | DVB subtitles ETSI EN 300 743 [2] (normal) for display |
|  |  |  | on 16:9 aspect ratio monitor |
|  |  | 0x13 | DVB subtitles ETSI EN 300 743 [2] (normal) for display |
|  |  |  | on 2.21:1 aspect ratio monitor |
|  |  | 0x14 | DVB subtitles ETSI EN 300 743 [2] (normal) for display |
|  |  |  | on a high definition monitor |
|  |  | 0x15 | DVB subtitles ETSI EN 300 743 [2] (normal) with plano- |
|  |  |  | stereoscopic disparity for display on a high definition |
|  |  |  | monitor |
|  |  | 0x16 | DVB subtitles ETSI EN 300 743 [2] (normal) for display |
|  |  |  | on an ultra high definition monitor |
|  |  | 0x17 to 0x1F | reserved for future use |
|  |  | 0x20 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) with no monitor aspect ratio criticality |
|  |  | 0x21 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) for display on 4:3 aspect ratio monitor |
|  |  | 0x22 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) for display on 16:9 aspect ratio monitor |
|  |  | 0x23 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) for display on 2.21:1 aspect ratio monitor |
|  |  | 0x24 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) for display on a high definition monitor |
|  |  | 0x25 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) with plano-stereoscopic disparity for display on a |
|  |  |  | high definition monitor |
|  |  | 0x26 | DVB subtitles ETSI EN 300 743 [2] (for the hard of |
|  |  |  | hearing) for display on an ultra high definition monitor |
|  |  | 0x27 to 0x2F | reserved for future use |
|  |  | 0x30 | open (in-vision) sign language interpretation for the deaf |
|  |  |  | (see note 4) |
|  |  | 0x31 | closed sign language interpretation for the deaf (see |
|  |  |  | note 4) |
|  |  | 0x32 to 0x3F | reserved for future use |
|  |  | 0x40 | video spatial resolution has been upscaled from lower |
|  |  |  | resolution source material (see note 4) |
|  |  | 0x41 | video is Standard Dynamic Range (SDR) (see note 4) |
|  |  | 0x42 | video is High Dynamic Range (HDR) remapped from |
|  |  |  | SDR source material (see note 4) |
|  |  | 0x43 | video is HDR up-converted from SDR source material |
|  |  |  | (see note 4) |
|  |  | 0x44 | video is standard frame rate, less than or equal to 60 Hz |
|  |  |  | (see note 4) |
|  |  | 0x45 | high frame rate video generated from lower frame rate |
|  |  |  | source material (see note 4) |
|  |  | 0x46 to 0x7F | reserved for future use |
|  |  | 0x80 | dependent Spatial Audio Object Coding – Dialogue |
|  |  |  | Enhancement (SAOC-DE) data stream |
|  |  | 0x81 to 0xAF | reserved for future use |
|  |  | 0xB0 to 0xFE | user defined |
|  |  | 0xFF | reserved for future use |
| 0x4 | n/a (see note 1) | 0x00 to 0x7F | reserved for AC-3 audio modes (refer to table D.1) |
|  |  | 0x80 to 0xFF | reserved for enhanced AC-3 audio modes (refer to |
|  |  |  | table D.1) |
| 0x5 | n/a (see note 1) | 0x00 | reserved for future use |
|  |  | 0x01 | H.264/AVC standard definition video, 4:3 aspect ratio, |
|  |  |  | 25 Hz (see note 2) |
|  |  | 0x02 | reserved for future use |
|  |  | 0x03 | H.264/AVC standard definition video, 16:9 aspect ratio, |
|  |  |  | 25 Hz (see note 2) |
|  |  | 0x04 | H.264/AVC standard definition video, > 16:9 aspect ratio, |
|  |  |  | 25 Hz (see note 2) |
|  |  | 0x05 | H.264/AVC standard definition video, 4:3 aspect ratio, |
|  |  |  | 30 Hz (see note 2) |
|  |  | 0x06 | reserved for future use |
|  |  | 0x07 | H.264/AVC standard definition video, 16:9 aspect ratio, |
|  |  |  | 30 Hz (see note 2) |
|  |  | 0x08 | H.264/AVC standard definition video, > 16:9 aspect ratio, |
|  |  |  | 30 Hz (see note 2) |
|  |  | 0x09 to 0x0A | reserved for future use |
|  |  | 0x0B | H.264/AVC high definition video, 16:9 aspect ratio, 25 Hz |
|  |  |  | (see note 2) |
|  |  | 0x0C | H.264/AVC high definition video, > 16:9 aspect ratio, |
|  |  |  | 25 Hz (see note 2) |
|  |  | 0x0D to 0x0E | reserved for future use |
|  |  | 0x0F | H.264/AVC high definition video, 16:9 aspect ratio, 30 Hz |
|  |  |  | (see note 2) |
|  |  | 0x10 | H.264/AVC high definition video, > 16:9 aspect ratio, |
|  |  |  | 30 Hz (see note 2) |
|  |  | 0x11 to 0x7F | reserved for future use |
|  |  | 0x80 | H.264/AVC plano-stereoscopic frame compatible high |
|  |  |  | definition video, 16:9 aspect ratio, 25 Hz, Side-by-Side |
|  |  |  | (see note 2 and note 5) |
|  |  | 0x81 | H.264/AVC plano-stereoscopic frame compatible high |
|  |  |  | definition video, 16:9 aspect ratio, 25 Hz, |
|  |  |  | Top-and-Bottom (see note 2 and note 5) |
|  |  | 0x82 | H.264/AVC plano-stereoscopic frame compatible high |
|  |  |  | definition video, 16:9 aspect ratio, 30 Hz, Side-by-Side |
|  |  |  | (see note 2, note 5, and note 6) |
|  |  | 0x83 | H.264/AVC stereoscopic frame compatible high definition |
|  |  |  | video, 16:9 aspect ratio, 30 Hz, Top-and-Bottom (see |
|  |  |  | note 2, note 5 and note 6) |
|  |  | 0x84 | H.264/MVC dependent view, plano-stereoscopic service |
|  |  |  | compatible video (see note 5) |
|  |  | 0x85 to 0xAF | reserved for future use |
|  |  | 0xB0 to 0xFE | user defined |
|  |  | 0xFF | reserved for future use |
| 0x6 | n/a (see note 1) | 0x00 | reserved for future use |
|  |  | 0x01 | High Efficiency AAC (HE-AAC) audio, single mono |
|  |  |  | channel (see note 7) |
|  |  | 0x02 | reserved for future use |
|  |  | 0x03 | HE-AAC audio, stereo (see note 7) |
|  |  | 0x04 | reserved for future use |
|  |  | 0x05 | HE-AAC audio, surround sound (see note 7) |
|  |  | 0x06 to 0x3F | reserved for future use |
|  |  | 0x40 | HE-AAC audio description for the visually impaired (see |
|  |  |  | note 8 and note 7) |
|  |  | 0x41 | HE-AAC audio for the hard of hearing (see note 7) |
|  |  | 0x42 | HE-AAC receiver-mix supplementary audio as per |
|  |  |  | annex E of ETSI TS 101 154 [14] (see note 7) |
|  |  | 0x43 | HE-AAC v2 audio, stereo |
|  |  | 0x44 | HE-AAC v2 audio description for the visually impaired |
|  |  |  | (see note 8) |
|  |  | 0x45 | HE-AAC v2 audio for the hard of hearing |
|  |  | 0x46 | HE-AAC v2 receiver-mix supplementary audio as per |
|  |  |  | annex E of ETSI TS 101 154 [14] |
|  |  | 0x47 | HE-AAC receiver-mix audio description for the visually |
|  |  |  | impaired |
|  |  | 0x48 | HE-AAC broadcast-mix audio description for the visually |
|  |  |  | impaired |
|  |  | 0x49 | HE-AAC v2 receiver-mix audio description for the visually |
|  |  |  | impaired |
|  |  | 0x4A | HE-AAC v2 broadcast-mix audio description for the |
|  |  |  | visually impaired |
|  |  | 0x4B to 0x9F | reserved for future use |
|  |  | 0xA0 | HE-AAC, or HE-AAC v2 with SAOC-DE ancillary data |
|  |  |  | (see note 7 and note 4) |
|  |  | 0xA1 to 0xAF | reserved for future use |
|  |  | 0xB0 to 0xFE | user defined |
|  |  | 0xFF | reserved for future use |
| 0x7 | n/a (see note 1) | 0x00 to 0x7F | reserved for DTS and DTS-HD audio modes (refer to |
|  |  |  | annex G) |
|  |  | 0x80 to 0xFF | reserved for future use |
| 0x8 | n/a (see note 1)a | 0x00 | reserved for future use |
|  |  | 0x01 | DVB System Renewability Message (SRM) data ETSI |
|  |  |  | TS 102 770 [22] |
|  |  | 0x02 to 0xFF | reserved for future use for DVB Content Protection Copy |
|  |  |  | Management (CPCM) modes ETSI TS 102 825 (parts 1 |
|  |  |  | to 5, 7, 9 and 10) [27], ETSI TR 102 825 (parts 6, 8, 11 to |
|  |  |  | 13) [i.5] |
| 0x9 | 0x0 | 0x00 | HEVC Main Profile high definition video, 50 Hz (see |
|  |  |  | note 2 and note 9) |
|  |  | 0x01 | HEVC Main 10 Profile high definition video, 50 Hz (see |
|  |  |  | note 2 and note 9) |
|  |  | 0x02 | HEVC Main Profile high definition video, 60 Hz (see |
|  |  |  | note 2, note 6 and note 9) |
|  |  | 0x03 | HEVC Main 10 Profile high definition video, 60 Hz (see |
|  |  |  | note 2, note 6 and note 9) |
|  |  | 0x04 | HEVC ultra high definition video, with a resolution up to |
|  |  |  | 3 840 x 2 160 (see note 2 and note 9, and see note 10) |
|  |  |  | conformant to one of the following Ultra High Definition |
|  |  |  | TeleVision (UHDTV) bit stream conformance points |
|  |  |  | defined in table 18b in ETSI TS 101 154 [14]: |
|  |  |  | • "SDR frame rate up to 60 Hz resolution up to |
|  |  |  | 3 840 x 2 160"; |
|  |  |  | • "SDR HFR dual PID with temporal scalability |
|  |  |  | resolution up to 3 840 x 2 160"; |
|  |  |  | • "HDR with HLG10 frame rate up to 60 Hz |
|  |  |  | resolution up to 3 840 x 2 160"; |
|  |  |  | • "HDR with HLG10 HFR dual PID and temporal |
|  |  |  | scalability resolution up to 3 840 x 2 160". |
|  |  | 0x05 | HEVC ultra high definition video with Perceptual |
|  |  |  | Quantizer with a bit depth of 10 bits (PQ10) HDR with a |
|  |  |  | frame rate lower than or equal to 60 Hz (see note 2 and |
|  |  |  | note 11) conformant to one of the following UHDTV bit |
|  |  |  | stream conformance points defined in table 18b in ETSI |
|  |  |  | TS 101 154 [14]: |
|  |  |  | • "HDR with PQ10 frame rate up to 60 Hz |
|  |  |  | resolution up to 3 840 x 2 160" |
|  |  |  | or |
|  |  |  | HEVC ultra high definition video with PQ10 HDR HDR |
|  |  |  | (cid:2)(cid:3)(cid:0)(cid:0)(cid:0)(cid:0) |
|  |  |  | with a frame rate of 100 Hz, (cid:2)(cid:0)(cid:0)(cid:2) Hz, or 120 Hz with a |
|  |  |  | half frame rate HEVC temporal video sub-bit-stream (see |
|  |  |  | note 2 and note 11) conformant to one of the following |
|  |  |  | UHDTV bit stream conformance points defined in table |
|  |  |  | 18b in ETSI TS 101 154 [14]: |
|  |  |  | • "HDR with PQ10 HFR dual PID and temporal |
|  |  |  | scalability resolution up to 3 840 x 2 160" |
|  |  | 0x06 | HEVC ultra high definition video, with a resolution up to |
|  |  |  | (cid:2)(cid:3)(cid:0)(cid:0)(cid:0)(cid:0) |
|  |  |  | 3 840 x 2 160, frame rate of 100 Hz, (cid:2)(cid:0)(cid:0)(cid:2) Hz, or |
|  |  |  | 120 Hz without a half frame rate HEVC temporal video |
|  |  |  | sub-bit-stream (see note 2 and note 11) conformant to |
|  |  |  | one of the following UHDTV bit stream conformance |
|  |  |  | points defined in table 18b in ETSI TS 101 154 [14]: |
|  |  |  | • "SDR HFR single PID resolution up to |
|  |  |  | 3 840 x 2 160"; |
|  |  |  | • "HDR with HLG10 HFR single PID resolution up |
|  |  |  | to 3 840 x 2 160" |
|  |  | 0x07 | HEVC ultra high definition video with PQ10 HDR, frame |
|  |  |  | (cid:2)(cid:3)(cid:0)(cid:0)(cid:0)(cid:0) |
|  |  |  | rate of 100 Hz, (cid:2)(cid:0)(cid:0)(cid:2) Hz, or 120 Hz without a half frame |
|  |  |  | rate HEVC temporal video sub-bit-stream (see note 2, |
|  |  |  | and see note 11) conformant to one of the following |
|  |  |  | UHDTV bit stream conformance points defined in |
|  |  |  | table 18b in ETSI TS 101 154 [14]: |
|  |  |  | • "PQ10 HFR single PID resolution up to |
|  |  |  | 3 840 x 2 160" |
|  |  | 0x08 | HEVC ultra high definition video with a resolution up to |
|  |  |  | 7 680 x 4 320 (see note 2, note 6 and note 11) |
|  |  |  | conformant to one of the following UHDTV2 bit stream |
|  |  |  | conformance point defined in Table 18b in ETSI |
|  |  |  | TS 101 154 [14]: |
|  |  |  | • "SDR frame rate up to 60 Hz resolution up to |
|  |  |  | 7 680 x 4 320"; |
|  |  |  | • "HDR with PQ10 frame rate up to 60 Hz |
|  |  |  | resolution up to 7 680 x 4 320"; |
|  |  |  | • "HDR with HLG10 frame rate up to 60 Hz |
|  |  |  | resolution up to 7 680 x 4 320" |
|  |  | 0x09 to 0x0F | reserved for future use for HEVC |
|  |  | 0x10 | VVC Main 10 Profile with resolution up to 3 840 x 2 160 |
|  |  |  | frame rate up to 60 Hz conformant to the VVC HDR |
|  |  |  | UHDTV-1 bitstream conformance point defined in |
|  |  |  | clause 5.15.2 in ETSI TS 101 154 [14] (see note 14) |
|  |  | 0x11 | VVC Main 10 Profile with resolution up to 3 840 x 2 160 |
|  |  |  | High Frame Rate of 100 Hz or 120 Hz conformant to the |
|  |  |  | VVC HDR HFR UHDTV-1 bitstream conformance point |
|  |  |  | defined in clause 5.15.3 in ETSI TS 101 154 [14] (see |
|  |  |  | note 14) |
|  |  | 0x12 | VVC Main 10 Profile with resolution up to 7 680 x 4 320 |
|  |  |  | frame rate up to 60 Hz conformant to the VVC HDR |
|  |  |  | UHDTV-2 bitstream conformance point defined in |
|  |  |  | clause 5.15.4 in ETSI TS 101 154 [14] (see note 14) |
|  |  | 0x13 | VVC Main 10 Profile with resolution up to 7 680 x 4 320 |
|  |  |  | High Frame Rate of 100 Hz or 120 Hz conformant to the |
|  |  |  | VVC HDR HFR UHDTV-2 bitstream conformance point |
|  |  |  | defined in clause 5.15.5 in ETSI TS 101 154 [14] (see |
|  |  |  | note 14) |
|  |  | 0x14 to 0x1F | reserved for future use for VVC |
|  |  | 0x20 | AVS3 High 10 Profile with resolution up to 3 840 x 2 160 |
|  |  |  | frame rate up to 60 Hz conformant to the AVS3 HDR |
|  |  |  | UHDTV-1 bitstream conformance point defined in |
|  |  |  | clause 5.16.3 in ETSI TS 101 154 [14] (see note 15) |
|  |  | 0x21 | AVS3 High 10 Profile with resolution up to 3 840 x 2 160 |
|  |  |  | High Frame Rate of 100 Hz or 120 Hz conformant to the |
|  |  |  | AVS3 HDR HFR UHDTV-1 bitstream conformance point |
|  |  |  | defined in clause 5.16.4 in ETSI TS 101 154 [14] (see |
|  |  |  | note 15) |
|  |  | 0x22 | AVS3 High 10 Profile with resolution up to 7 680 x 4 320 |
|  |  |  | frame rate up to 60 Hz conformant to the AVS3 HDR |
|  |  |  | UHDTV-2 bitstream conformance point defined in |
|  |  |  | clause 5.16.5 in ETSI TS 101 154 [14] (see note 15) |
|  |  | 0x23 | AVS3 High 10 Profile with resolution up to 7 680 x 4 320 |
|  |  |  | High Frame Rate of 100 Hz or 120 Hz conformant to the |
|  |  |  | AVS3 HDR HFR UHDTV-2 bitstream conformance point |
|  |  |  | defined in clause 5.16.6 in ETSI TS 101 154 [14] (see |
|  |  |  | note 15) |
|  |  | 0x24 to 0x2F | reserved for future use for AVS3 |
|  |  | 0x30 to 0xFF | reserved for future use |
|  | 0x1 | 0x00 | AC-4 main audio, mono (see note 12) |
|  |  | 0x01 | AC-4 main audio, mono, dialogue enhancement enabled |
|  |  |  | (see note 12) |
|  |  | 0x02 | AC-4 main audio, stereo (see note 12) |
|  |  | 0x03 | AC-4 main audio, stereo, dialogue enhancement enabled |
|  |  |  | (see note 12) |
|  |  | 0x04 | AC-4 main audio, multichannel (see note 12) |
|  |  | 0x05 | AC-4 main audio, multichannel, dialogue enhancement |
|  |  |  | enabled (see note 12) |
|  |  | 0x06 | AC-4 broadcast-mix audio description, mono, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x07 | AC-4 broadcast-mix audio description, mono, for the |
|  |  |  | visually impaired, dialogue enhancement enabled (see |
|  |  |  | note 12) |
|  |  | 0x08 | AC-4 broadcast-mix audio description, stereo, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x09 | AC-4 broadcast-mix audio description, stereo, for the |
|  |  |  | visually impaired, dialogue enhancement enabled (see |
|  |  |  | note 12) |
|  |  | 0x0A | AC-4 broadcast-mix audio description, multichannel, for |
|  |  |  | the visually impaired (see note 12) |
|  |  | 0x0B | AC-4 broadcast-mix audio description, multichannel, for |
|  |  |  | the visually impaired, dialogue enhancement enabled |
|  |  |  | (see note 12) |
|  |  | 0x0C | AC-4 receiver-mix audio description, mono, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x0D | AC-4 receiver-mix audio description, stereo, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x0E | AC-4 Part-2 |
|  |  | 0x0F | MPEG-H Audio Low Complexity (LC) Profile |
|  |  | 0x10 | DTS-UHD main audio, mono (see note 12) |
|  |  | 0x11 | DTS-UHD main audio, mono, dialogue enhancement |
|  |  |  | enabled (see note 12) |
|  |  | 0x12 | DTS-UHD main audio, stereo (see note 12) |
|  |  | 0x13 | DTS-UHD main audio, stereo, dialogue enhancement |
|  |  |  | enabled (see note 12) |
|  |  | 0x14 | DTS-UHD main audio, multichannel (see note 12) |
|  |  | 0x15 | DTS-UHD main audio, multichannel, dialogue |
|  |  |  | enhancement enabled (see note 12) |
|  |  | 0x16 | DTS-UHD broadcast-mix audio description, mono, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x17 | DTS-UHD broadcast-mix audio description, mono, for the |
|  |  |  | visually impaired, dialogue enhancement enabled (see |
|  |  |  | note 12) |
|  |  | 0x18 | DTS-UHD broadcast-mix audio description, stereo, for |
|  |  |  | the visually impaired (see note 12) |
|  |  | 0x19 | DTS-UHD broadcast-mix audio description, stereo, for |
|  |  |  | the visually impaired, dialogue enhancement enabled |
|  |  |  | (see note 12) |
|  |  | 0x1A | DTS-UHD broadcast-mix audio description, multichannel, |
|  |  |  | for the visually impaired (see note 12) |
|  |  | 0x1B | DTS-UHD broadcast-mix audio description, multichannel, |
|  |  |  | for the visually impaired, dialogue enhancement enabled |
|  |  |  | (see note 12) |
|  |  | 0x1C | DTS-UHD receiver-mix audio description, mono, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x1D | DTS-UHD receiver-mix audio description, stereo, for the |
|  |  |  | visually impaired (see note 12) |
|  |  | 0x1E | DTS-UHD Next Generation Audio (NGA) Audio |
|  |  | 0x1F to 0xFF | reserved for future use |
|  | 0x2 | 0x00 to 0xFF | Timed Text Markup Language (TTML) subtitles ETSI |
|  |  |  | EN 303 560 [12] |
|  | 0x3 to 0xF | 0x00 to 0xFF | reserved for future use |
| 0xA | 0x0 to 0xF | 0x00 to 0xFF | reserved for future use |
| 0xB | 0x0 to 0xD | 0x00 to 0xFF | reserved for future use |
|  | 0xE | 0x00 to 0xFF | NGA component type feature flags according to table 27 |
|  | 0xF (see note 4) | 0x00 | less than 16:9 aspect ratio |
|  |  | 0x01 | 16:9 aspect ratio |
|  |  | 0x02 | greater than 16:9 aspect ratio |
|  |  | 0x03 | plano-stereoscopic top and bottom (TaB) frame-packing |
|  |  | 0x04 | HLG10 HDR (see note 4, note 11 and note 13) |
|  |  | 0x05 | HEVC temporal video subset for a frame rate of 100 Hz, |
|  |  |  | (cid:2)(cid:3)(cid:0)(cid:0)(cid:0)(cid:0) |
|  |  |  | (cid:2)(cid:0)(cid:0)(cid:2) Hz, or 120 Hz (see note 11 and note 13) |
|  |  | 0x06 | SMPTE ST 2094-10 Dynamic Mapping Information (DMI) |
|  |  |  | format as defined in clause 5.14.4.4.3.4.3 of ETSI |
|  |  |  | TS 101 154 [14] (see note 4 and note 11) |
|  |  | 0x07 | SL-HDR2 DMI format as defined in clause 5.14.4.4.3.4.4 |
|  |  |  | of ETSI TS 101 154 [14] (see note 4 and note 11) |
|  |  | 0x08 | SMPTE ST 2094-40 DMI format as defined in |
|  |  |  | clause 5.14.4.4.3.4.5 of ETSI TS 101 154 [14] (see |
|  |  |  | note 4 and note 11) |
|  |  | 0x09 | PQ10 HDR (see note 4) |
|  |  | 0x0A to 0xFF | reserved for future use |

### Table 27 — NGA component_type value assignments
_PDF pages 67-67 (§6.2.9)_

| stream_content | stream_content_ext | component_type | Description |
|---|---|---|---|
| 0xC to 0xF | n/a | 0x00 to 0xFF | user defined |
| NOTE 1: In order to maintain backwards compatibility, the value of the stream_content_ext field is not applicable (n/a) |  |  |  |
| for stream_content values in the range 0x1 to 0x8, and is set to 0xF. |  |  |  |
| NOTE 2: The terms "standard definition", "high definition", "ultra high definition", "25 Hz", "30 Hz", "50 Hz", and |  |  |  |
| "60 Hz" are used as defined in ETSI TS 101 154 [14], clauses 5.1 to 5.4 for MPEG-2 and clauses 5.5 to 5.7 |  |  |  |
| for H.264/AVC, and clauses 5.14.2 and 5.14.3 for HEVC respectively. The terms "HDR", "HLG10" and |  |  |  |
| "PQ10" are used as defined in clause 5.14.3 of ETSI TS 101 154 [14]. |  |  |  |
| NOTE 3: The specific audio description types indicating the use of broadcast-mix or receiver-mix audio should be |  |  |  |
| preferred over these generic types. For more details see annex J. |  |  |  |
| NOTE 4: These component_descriptor values are intended to be present in combination with one or more |  |  |  |
| component descriptors with the same component_tag value. See annex N for example uses. |  |  |  |
| NOTE 5: See ETSI TS 101 547-3 [17] for further information on stereoscopic modes. |  |  |  |
| NOTE 6: 24 Hz video will also use this component_type. |  |  |  |
| NOTE 7: Audio streams using AAC audio shall use the corresponding HE AAC values. The AAC profile includes low- |  |  |  |
| complexity AAC. |  |  |  |
| NOTE 8: The specific audio description types indicating the use of broadcast-mix or receiver-mix audio should be |  |  |  |
| preferred over these generic types. For more details see annex J. |  |  |  |
| NOTE 9: For rules on the use of these values, see clause I.2.5 and ETSI TS 101 547-4 [18]. |  |  |  |
| NOTE 10: This value should be used for backward compatible Hybrid Log Gamma with a bit depth of 10 bits (HLG10) |  |  |  |
| HDR services, and/or backward compatible High Frame Rate (HFR) services which are decodable by |  |  |  |
| HEVC_UHDTV_IRD as defined in ETSI TS 101 154 [14]. See also clause I.2.5.2. |  |  |  |
| NOTE 11: For the rules on the use of these values, see clause I.2.6. |  |  |  |
| NOTE 12: These values should be used for elementary streams that convey a single presentation only. |  |  |  |
| NOTE 13: For the rules on the use of these values, see clause I.2.5.2. |  |  |  |
| NOTE 14: For the rules on the use of these values, see clause I.2.8. |  |  |  |
| NOTE 15: For the rules on the use of these values, see clause I.2.9. |  |  |  |
| component_type bits |  | Description |  |
| b(cid:8) (msb) |  | reserved_zero_future_use |  |
| b(cid:7) |  | content is pre-rendered for consumption with headphones |  |
| b(cid:6) |  | content enables interactivity |  |
| b(cid:5) |  | content enables dialogue enhancement (see note) |  |
| b(cid:4) |  | content contains spoken subtitles |  |
| b(cid:3) |  | content contains audio description |  |
| b(cid:2) | b(cid:0) | preferred reproduction channel layout |  |
| 0b0 | 0b0 | no preference |  |
| 0b0 | 0b1 | stereo |  |
| 0b1 | 0b0 | two-dimensional |  |
| 0b1 | 0b1 | three-dimensional |  |
| NOTE: Content enabling dialogue enhancement also offers support for clean audio for the hearing impaired. |  |  |  |

### Table 28 — Content descriptor
_PDF pages 68-68 (§6.2.9)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| content_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | uimsbf |
| for (i=0;i<N;i++) { | 4 | uimsbf |
| content_nibble_level_1 | 8 | uimsbf |
| content_nibble_level_2 |  |  |
| user_byte |  |  |
| } |  |  |
| } |  |  |

### Table 29 — Content nibble level 1 and 2 assignments
_PDF pages 68-69 (§6.2.9)_

| content_nibble_level_1 | content_nibble_level_2 | Description |  |
|---|---|---|---|
| 0x0 | 0x0 to 0xF | undefined content |  |
| Movie/Drama: |  |  |  |
| 0x1 | 0x0 | movie/drama (general) |  |
| 0x1 | 0x1 | detective/thriller |  |
| 0x1 | 0x2 | adventure/western/war |  |
| 0x1 | 0x3 | science fiction/fantasy/horror |  |
| 0x1 | 0x4 | comedy |  |
| 0x1 | 0x5 | soap/melodrama/folkloric |  |
| 0x1 | 0x6 | romance |  |
| 0x1 | 0x7 | serious/classical/religious/historical movie/drama |  |
| 0x1 | 0x8 | adult movie/drama |  |
| 0x1 | 0x9 to 0xE | reserved for future use |  |
| 0x1 | 0xF | user defined |  |
| News/Current affairs: |  |  |  |
| 0x2 | 0x0 | news/current affairs (general) |  |
| 0x2 | 0x1 | news/weather report |  |
| 0x2 | 0x2 | news magazine |  |
| 0x2 | 0x3 | documentary |  |
| 0x2 | 0x4 | discussion/interview/debate |  |
| 0x2 | 0x5 to 0xE | reserved for future use |  |
| 0x2 | 0xF | user defined |  |
| Show/Game show: |  |  |  |
| 0x3 | 0x0 | show/game show (general) |  |
| 0x3 | 0x1 | game show/quiz/contest |  |
| 0x3 | 0x2 | variety show |  |
| 0x3 | 0x3 | talk show |  |
| 0x3 | 0x4 to 0xE | reserved for future use |  |
| 0x3 | 0xF | user defined |  |
| Sports: |  |  |  |
| 0x4 | 0x0 | sports (general) |  |
| 0x4 | 0x1 | special events (Olympic Games, World Cup, etc.) |  |
| 0x4 | 0x2 | sports magazines |  |
| 0x4 | 0x3 | football/soccer |  |
| 0x4 | 0x4 | tennis/squash |  |
| 0x4 | 0x5 | team sports (excluding football) |  |
| 0x4 | 0x6 | athletics |  |
| 0x4 | 0x7 | motor sport |  |
| 0x4 | 0x8 | water sport |  |
| 0x4 | 0x9 | winter sports |  |
| 0x4 | 0xA | equestrian |  |
| 0x4 | 0xB | martial sports |  |
| 0x4 | 0xC to 0xE | reserved for future use |  |
| 0x4 | 0xF | user defined |  |
| Children's/Youth programmes: |  |  |  |
| 0x5 | 0x0 | children's/youth programmes (general) |  |
| 0x5 | 0x1 | pre-school children's programmes |  |
| 0x5 | 0x2 | entertainment programmes for 6 to 14 |  |
| 0x5 | 0x3 | entertainment programmes for 10 to 16 |  |
| 0x5 | 0x4 | informational/educational/school programmes |  |
| 0x5 | 0x5 | cartoons/puppets |  |
| 0x5 | 0x6 to 0xE | reserved for future use |  |
| 0x5 | 0xF | user defined |  |
| Music/Ballet/Dance: |  |  |  |
| 0x6 | 0x0 | music/ballet/dance (general) |  |
| 0x6 | 0x1 | rock/pop |  |
| 0x6 | 0x2 | serious music/classical music |  |
| 0x6 | 0x3 | folk/traditional music |  |
| 0x6 | 0x4 | jazz |  |
| 0x6 | 0x5 | musical/opera |  |
| 0x6 | 0x6 | ballet |  |
| 0x6 | 0x7 to 0xE | reserved for future use |  |
| 0x6 | 0xF | user defined |  |
| Arts/Culture (without music): |  |  |  |
| 0x7 | 0x0 | arts/culture (without music, general) |  |
| 0x7 | 0x1 | performing arts |  |
| 0x7 | 0x2 | fine arts |  |
| 0x7 | 0x3 | religion |  |
| 0x7 | 0x4 | popular culture/traditional arts |  |
| 0x7 | 0x5 | literature |  |
| 0x7 | 0x6 | film/cinema |  |
| 0x7 | 0x7 | experimental film/video |  |
| 0x7 | 0x8 | broadcasting/press |  |
| 0x7 | 0x9 | new media |  |
| 0x7 | 0xA | arts/culture magazines |  |
| 0x7 | 0xB | fashion |  |
| 0x7 | 0xC to 0xE | reserved for future use |  |
| 0x7 | 0xF | user defined |  |
| Social/Political issues/Economics: |  |  |  |
| 0x8 | 0x0 | social/political issues/economics (general) |  |
| 0x8 | 0x1 | magazines/reports/documentary |  |
| 0x8 | 0x2 | economics/social advisory |  |
| 0x8 | 0x3 | remarkable people |  |
| 0x8 | 0x4 to 0xE | reserved for future use |  |
| 0x8 | 0xF | user defined |  |
| Education/Science/Factual topics: |  |  |  |
| 0x9 | 0x0 | education/science/factual topics (general) |  |
| 0x9 | 0x1 | nature/animals/environment |  |
| 0x9 | 0x2 | technology/natural sciences |  |
| 0x9 | 0x3 | medicine/physiology/psychology |  |
| 0x9 | 0x4 | foreign countries/expeditions |  |
| 0x9 | 0x5 | social/spiritual sciences |  |
| 0x9 | 0x6 | further education |  |
| 0x9 | 0x7 | languages |  |
| 0x9 | 0x8 to 0xE | reserved for future use |  |
| 0x9 | 0xF | user defined |  |
| Leisure hobbies: |  |  |  |
| 0xA | 0x0 | leisure hobbies (general) |  |
| 0xA | 0x1 | tourism/travel |  |
| 0xA | 0x2 | handicraft |  |
| 0xA | 0x3 | motoring |  |
| 0xA | 0x4 | fitness and health |  |

### Table 30 — Country availability descriptor
_PDF pages 70-70 (§6.2.10)_

| content_nibble_level_1 | content_nibble_level_2 | Description |  |
|---|---|---|---|
| 0xA | 0x5 | cooking |  |
| 0xA | 0x6 | advertisement/shopping |  |
| 0xA | 0x7 | gardening |  |
| 0xA | 0x8 to 0xE | reserved for future use |  |
| 0xA | 0xF | user defined |  |
| Special characteristics: |  |  |  |
| 0xB | 0x0 | original language |  |
| 0xB | 0x1 | black and white |  |
| 0xB | 0x2 | unpublished |  |
| 0xB | 0x3 | live broadcast |  |
| 0xB | 0x4 | plano-stereoscopic |  |
| 0xB | 0x5 | local or regional |  |
| 0xB | 0x6 to 0xE | reserved for future use |  |
| 0xB | 0xF | user defined |  |
| Adult: |  |  |  |
| 0xC | 0x0 | adult (general) |  |
| 0xC | 0x1 to 0xE | reserved for future use |  |
| 0xC | 0xF | user defined |  |
| Reserved for future use: |  |  |  |
| 0xD to 0xE | 0x0 to 0xF | reserved for future use |  |
| User defined: |  |  |  |
| 0xF | 0x0 to 0xF | user defined |  |
| Syntax | Number of bits | Identifier |  |
| country_availability_descriptor() { | 8 | uimsbf |  |
| descriptor_tag | 8 | uimsbf |  |
| descriptor_length | 1 | bslbf |  |
| country_availability_flag | 7 | bslbf |  |
| reserved_future_use | 24 | bslbf |  |
| for (i=0;i<N;i++) { |  |  |  |
| country_code |  |  |  |
| } |  |  |  |
| } |  |  |  |

### Table 31 — Data broadcast descriptor
_PDF pages 71-71 (§6.2.12)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| data_broadcast_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| data_broadcast_id | 8 | uimsbf |
| component_tag | 8 | uimsbf |
| selector_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 24 | bslbf |
| selector_byte | 8 | uimsbf |
| } | 8 | uimsbf |
| ISO_639_language_code |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 32 — Data broadcast id descriptor
_PDF pages 72-72 (§6.2.13.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| data_broadcast_id_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| data_broadcast_id | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| selector_byte |  |  |
| } |  |  |
| } |  |  |

### Table 33 — Cable delivery system descriptor
_PDF pages 72-72 (§6.2.13.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| cable_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | bslbf |
| frequency | 12 | bslbf |
| reserved_future_use | 4 | bslbf |
| FEC_outer | 8 | bslbf |
| modulation | 28 | bslbf |
| symbol_rate | 4 | bslbf |
| FEC_inner |  |  |
| } |  |  |

### Table 34 — Outer FEC scheme
_PDF pages 72-72 (§6.2.13.1)_

| FEC_outer | Description |
|---|---|
| 0b0000 | not defined |
| 0b0001 | no outer FEC coding |
| 0b0010 | (204,188) Reed-Solomon code (RS) |
| 0b0011 to 0b1111 | reserved for future use |

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

### Table 42 — S2 satellite delivery system descriptor
_PDF pages 75-75 (§6.2.13.3)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| S2_satellite_delivery_system_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| scrambling_sequence_selector | 1 | bslbf |
| multiple_input_stream_flag | 1 | bslbf |
| reserved_zero_future_use | 1 | bslbf |
| not_timeslice_flag | 2 | bslbf |
| reserved_future_use | 2 | uimsbf |
| TS_GS_mode | 6 | bslbf |
| if (scrambling_sequence_selector == 0b1) { | 18 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| scrambling_sequence_index | 8 | uimsbf |
| } |  |  |
| if (multiple_input_stream_flag == 0b1) { |  |  |
| input_stream_identifier |  |  |
| } |  |  |
| if (not_timeslice_flag == 0b0) { |  |  |
| timeslice_number |  |  |
| } |  |  |
| } |  |  |

### Table 43 — Coding of the TS GS mode
_PDF pages 75-75 (§6.2.13.3)_

| TS_GS_mode (see note) | Description |
|---|---|
| 0 | Generic Packetized |
| 1 | Generic Stream Encapsulation (GSE) |
| 2 | DVB transport stream |
| 3 | reserved for future use |
| NOTE: These values are different from similar assignments in table 3 in ETSI EN 302 307-1 [7]. |  |

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

### Table 53 — Extended event descriptor
_PDF pages 79-79 (§6.2.16)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| extended_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | uimsbf |
| descriptor_number | 4 | uimsbf |
| last_descriptor_number | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| length_of_items | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| item_description_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| item_description_char | 8 | uimsbf |
| } | 8 | uimsbf |
| item_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| item_char |  |  |
| } |  |  |
| } |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| text_char |  |  |
| } |  |  |
| } |  |  |

### Table 54 — Extension descriptor
_PDF pages 80-80 (§6.2.18.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| extension_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | bslbf |
| for (i=0;i<N;i++) { |  |  |
| selector_byte |  |  |
| } |  |  |
| } |  |  |

### Table 55 — Frequency list descriptor
_PDF pages 80-80 (§6.2.18.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| frequency_list_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 6 | bslbf |
| reserved_future_use | 2 | bslbf |
| coding_type | 32 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| centre_frequency |  |  |
| } |  |  |
| } |  |  |

### Table 56 — Coding type coding
_PDF pages 80-80 (§6.2.18.1)_

| coding_type | Description |
|---|---|
| 0b00 | not defined |
| 0b01 | satellite |
| 0b10 | cable |
| 0b11 | terrestrial |

### Table 57 — FTA content management descriptor
_PDF pages 82-82 (§6.2.18.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| FTA_content_management_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| user_defined | 3 | bslbf |
| reserved_future_use | 1 | uimsbf |
| do_not_scramble | 2 | uimsbf |
| control_remote_access_over_internet | 1 | uimsbf |
| do_not_apply_revocation |  |  |
| } |  |  |

### Table 58 — Coding of control_remote_access_over_internet
_PDF pages 82-82 (§6.2.18.1)_

| control_remote_access_over_internet | Description |
|---|---|
| 0b00 | Redistribution over the Internet is enabled. |
| 0b01 | Redistribution over the Internet is enabled but only within a managed |
|  | domain. |
| 0b10 | Redistribution over the Internet is enabled but only within a managed |
|  | domain and after a certain short period of time (e.g. 24 hours). |
| 0b11 | Redistribution over the Internet is not allowed with the following exception: |
|  | Redistribution over the Internet within a managed domain is enabled after a |
|  | specified long (possibly indefinite) period of time. |

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

### Table 69 — Local time offset descriptor
_PDF pages 89-89 (§6.2.20)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| local_time_offset_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 6 | bslbf |
| country_code | 1 | bslbf |
| country_region_id | 1 | bslbf |
| reserved_future_use | 16 | bslbf |
| local_time_offset_polarity | 40 | bslbf |
| local_time_offset | 16 | bslbf |
| time_of_change |  |  |
| next_time_offset |  |  |
| } |  |  |
| } |  |  |

### Table 70 — Country region id coding
_PDF pages 90-90 (§6.2.21)_

| country_region_id | Description |
|---|---|
| 0b00 0000 | no time zone extension used |
| 0b00 0001 | time zone 1 (most easterly region) |
| 0b00 0010 | time zone 2 |
| … | … |
| 0b11 1100 | time zone 60 |
| 0b11 1101 to 0b11 1111 | reserved for future use |

### Table 71 — Mosaic descriptor
_PDF pages 91-91 (§6.2.21)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| mosaic_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| mosaic_entry_point | 3 | uimsbf |
| number_of_horizontal_elementary_cells | 1 | bslbf |
| reserved_future_use | 3 | uimsbf |
| number_of_vertical_elementary_cells | 6 | uimsbf |
| for (i=0;i<N;i++) { | 7 | bslbf |
| logical_cell_id | 3 | uimsbf |
| reserved_future_use | 8 | uimsbf |
| logical_cell_presentation_info | 2 | bslbf |
| elementary_cell_field_length | 6 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| reserved_future_use | 16 | uimsbf |
| elementary_cell_id | 16 | uimsbf |
| } | 16 | uimsbf |
| cell_linkage_info | 16 | uimsbf |
| if (cell_linkage_info == 0x01) { | 16 | uimsbf |
| bouquet_id | 16 | uimsbf |
| } | 16 | uimsbf |
| if (cell_linkage_info == 0x02) { | 16 | uimsbf |
| original_network_id | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| service_id | 16 | uimsbf |
| } |  |  |
| if (cell_linkage_info == 0x03) { |  |  |
| original_network_id |  |  |
| transport_stream_id |  |  |
| service_id |  |  |
| } |  |  |
| if (cell_linkage_info == 0x04) { |  |  |
| original_network_id |  |  |
| transport_stream_id |  |  |
| service_id |  |  |
| event_id |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 72 — Number of horizontal elementary cells coding
_PDF pages 91-91 (§6.2.21)_

| number_of_horizontal_elementary_cells | Description |
|---|---|
| 0 | one cell |
| 1 | two cells |
| 2 | three cells |
| 3 | four cells |
| 4 | five cells |
| 5 | six cells |
| 6 | seven cells |
| 7 | eight cells |

### Table 73 — Number of vertical elementary cells coding
_PDF pages 92-92 (§6.2.21)_

| number_of_vertical_elementary_cells | Description |
|---|---|
| 0 | one cell |
| 1 | two cells |
| 2 | three cells |
| 3 | four cells |
| 4 | five cells |
| 5 | six cells |
| 6 | seven cells |
| 7 | eight cells |

### Table 74 — Logical cell presentation info coding
_PDF pages 92-92 (§6.2.21)_

| logical_cell_presentation_info | Description |
|---|---|
| 0 | undefined |
| 1 | video |
| 2 | still picture (see note) |
| 3 | graphics/text |
| 4 to 7 | reserved for future use |
| NOTE: A coded still picture consists of a video sequence containing exactly one |  |
| coded picture which is intra-coded. |  |

### Table 75 — Cell linkage info coding
_PDF pages 93-93 (§6.2.22)_

| cell_linkage_info | Description |
|---|---|
| 0x00 | undefined |
| 0x01 | bouquet related |
| 0x02 | service related |
| 0x03 | other mosaic related |
| 0x04 | event related |
| 0x05 to 0xFF | reserved for future use |

### Table 76 — Multilingual bouquet name descriptor
_PDF pages 93-93 (§6.2.22)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_bouquet_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| name_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 77 — Multilingual component name descriptor
_PDF pages 94-94 (§6.2.24)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_component_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| component_tag | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| text_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 78 — Multilingual network name descriptor
_PDF pages 94-94 (§6.2.24)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_network_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| name_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 79 — Multilingual service name descriptor
_PDF pages 95-95 (§6.2.26)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| multilingual_service_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| ISO_639_language_code | 8 | uimsbf |
| service_provider_name_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 8 | uimsbf |
| char |  |  |
| } |  |  |
| service_name_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 80 — NVOD reference descriptor
_PDF pages 96-96 (§6.2.28)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| NVOD_reference_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 16 | uimsbf |
| transport_stream_id | 16 | uimsbf |
| original_network_id |  |  |
| service_id |  |  |
| } |  |  |
| } |  |  |

### Table 81 — Network name descriptor
_PDF pages 96-96 (§6.2.28)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| network_name_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 82 — Parental rating descriptor
_PDF pages 97-97 (§6.2.30)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| parental_rating_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| country_code |  |  |
| rating |  |  |
| } |  |  |
| } |  |  |

### Table 83 — Parental rating coding
_PDF pages 97-97 (§6.2.30)_

| rating | Description |
|---|---|
| 0x00 | undefined |
| 0x01 to 0x0F | minimum age = rating + 3 years |
| 0x10 to 0xFF | defined by the broadcaster |

### Table 84 — PDC descriptor
_PDF pages 97-97 (§6.2.30)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| PDC_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | bslbf |
| reserved_future_use | 20 | bslbf |
| programme_identification_label |  |  |
| } |  |  |

### Table 85 — Private data specifier descriptor
_PDF pages 98-98 (§6.2.32)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| private_data_specifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 32 | uimsbf |
| private_data_specifier |  |  |
| } |  |  |

### Table 86 — Scrambling descriptor
_PDF pages 98-98 (§6.2.32)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| scrambling_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| scrambling_mode |  |  |
| } |  |  |

### Table 87 — Scrambling mode coding
_PDF pages 99-99 (§6.2.33)_

| scrambling_mode | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | this value indicates use of DVB-Common Scrambling Algorithm Version 1 (CSA1). It is the |
|  | default mode and shall be used when the scrambling_descriptor is not present in the |
|  | program map section |
| 0x02 | this value indicates use of DVB-Common Scrambling Algorithm Version 2 (CSA2) |
| 0x03 | this value indicates use of DVB-Common Scrambling Algorithm Version 3 (CSA3) |
| 0x04 to 0x0F | reserved for future use |
| 0x10 | this value indicates use of DVB-Common IPTV Software-oriented Scrambling Algorithm (CISSA) |
|  | version 1 |
| 0x11 to 0x1F | reserved for future use for DVB-CISSA versions |
| 0x20 to 0x6F | reserved for future use |
| 0x70 to 0x7F | Alliance for Telecommunications Industry Solutions (ATIS) defined (see annex J of |
|  | ATIS 0800006 [i.6]) |
| 0x80 to 0xFE | user defined |
| 0xFF | reserved for future use |

### Table 88 — Service descriptor
_PDF pages 99-99 (§6.2.33)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| service_type | 8 | uimsbf |
| service_provider_name_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| char | 8 | uimsbf |
| } |  |  |
| service_name_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| char |  |  |
| } |  |  |
| } |  |  |

### Table 89 — Service type coding
_PDF pages 100-100 (§6.2.33)_

| service_type | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | digital television service (see note 1) |
| 0x02 | digital radio sound service (see note 2) |
| 0x03 | teletext service |
| 0x04 | NVOD reference service (see note 1) |
| 0x05 | NVOD time-shifted service (see note 1) |
| 0x06 | mosaic service |
| 0x07 | Frequency Modulation (FM) radio service |
| 0x08 | DVB SRM service (ETSI TS 102 770 [22]) |
| 0x09 | reserved for future use |
| 0x0A | advanced codec digital radio sound service |
| 0x0B | H.264/Advanced Video Coding (AVC) mosaic service |
| 0x0C | data broadcast service |
| 0x0D | reserved for Common Interface (CI) usage (EN 50221 [31]) |
| 0x0E | RCS Map (ETSI EN 301 790 [6]) |
| 0x0F | RCS Forward Link Signalling (FLS) (ETSI EN 301 790 [6]) |
| 0x10 | DVB Multimedia Home Platform (MHP) service |
| 0x11 | HD digital television service |
| 0x12 to 0x15 | reserved for future use |
| 0x16 | H.264/AVC SD digital television service |
| 0x17 | H.264/AVC SD NVOD time-shifted service |
| 0x18 | H.264/AVC SD NVOD reference service |
| 0x19 | H.264/AVC HD digital television service |
| 0x1A | H.264/AVC HD NVOD time-shifted service |
| 0x1B | H.264/AVC HD NVOD reference service |
| 0x1C | H.264/AVC frame compatible plano-stereoscopic HD digital |
|  | television service (see note 3) |
| 0x1D | H.264/AVC frame compatible plano-stereoscopic HD NVOD |
|  | time-shifted service (see note 3) |
| 0x1E | H.264/AVC frame compatible plano-stereoscopic HD NVOD |
|  | reference service (see note 3) |
| 0x1F | HEVC digital television service (see note 4) |
| 0x20 | HEVC UHD digital television service (see note 5) with either: |
|  | • a resolution up to 3 840 x 2 160, HDR and/or a frame rate |
|  | (cid:2)(cid:3)(cid:0)(cid:0)(cid:0)(cid:0) |
|  | of 100 Hz, (cid:2)(cid:0)(cid:0)(cid:2) Hz or 120 Hz, |
|  | • or a resolution greater than 3 840 x 2 160, SDR or HDR, |
|  | with a frame rate up to 60 Hz. |
| 0x21 | VVC digital television service (see note 6) |
| 0x22 | AVS3 digital television service (see note 7) |
| 0x23 to 0x7F | reserved for future use |
| 0x80 to 0xFE | user defined |
| 0xFF | reserved for future use |
| NOTE 1: MPEG-2 SD material should use this type. |  |
| NOTE 2: MPEG-1 Layer 2 audio material should use this type. |  |
| NOTE 3: For information on the use of these values, see clause I.2.3 and ETSI |  |
| TS 101 547-2 [16]. |  |
| NOTE 4: For rules on the use of this value, see clause I.2.5 and ETSI |  |
| TS 101 547-4 [18]. This value should be used for backward compatible |  |
| HLG10 HDR services, and/or backward compatible HFR services which are |  |
| decodable by HEVC_UHDTV_IRD as defined in ETSI TS 101 154 [14], see |  |
| clause I.2.5.2. |  |
| NOTE 5: For rules on the use of these values, see clause I.2.6. |  |
| NOTE 6: For rules on the use of these values, see clause I.2.8. |  |
| NOTE 7: For rules on the use of these values, see clause I.2.9. |  |

### Table 90 — Service availability descriptor
_PDF pages 101-101 (§6.2.36)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_availability_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 1 | bslbf |
| availability_flag | 7 | bslbf |
| reserved_future_use | 16 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| cell_id |  |  |
| } |  |  |
| } |  |  |

### Table 91 — Service list descriptor
_PDF pages 101-101 (§6.2.36)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_list_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| service_id |  |  |
| service_type |  |  |
| } |  |  |
| } |  |  |

### Table 92 — Service move descriptor
_PDF pages 102-102 (§6.2.37)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| service_move_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| new_original_network_id | 16 | uimsbf |
| new_transport_stream_id | 16 | uimsbf |
| new_service_id |  |  |
| } |  |  |

### Table 93 — Short event descriptor
_PDF pages 102-102 (§6.2.37)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| short_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| name_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| char | 8 | uimsbf |
| } |  |  |
| text_length |  |  |
| for (i=0;i<N;i++) { |  |  |
| text_char |  |  |
| } |  |  |
| } |  |  |

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

### Table 97 — Stream identifier descriptor
_PDF pages 105-105 (§6.2.41)_

| sb_leak_rate | Leak rate (Mbit/s) |
|---|---|
| 60 | 108,0 |
| 61 to 63 | reserved for future use |

### Table 98 — Stuffing descriptor
_PDF pages 105-105 (§6.2.41)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| stream_identifier_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| component_tag |  |  |
| } |  |  |
| stuffing_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | bslbf |
| for (i=0;i<N;i++) { |  |  |
| stuffing_byte |  |  |
| } |  |  |
| } |  |  |

### Table 99 — Subtitling descriptor
_PDF pages 106-106 (§6.2.42)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| subtitling_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 8 | bslbf |
| ISO_639_language_code | 16 | bslbf |
| subtitling_type | 16 | bslbf |
| composition_page_id |  |  |
| ancillary_page_id |  |  |
| } |  |  |
| } |  |  |

### Table 100 — Telephone descriptor
_PDF pages 107-107 (§6.2.42)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| telephone_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 2 | bslbf |
| reserved_future_use | 1 | bslbf |
| foreign_availability | 5 | uimsbf |
| connection_type | 1 | bslbf |
| reserved for future use | 2 | uimsbf |
| country_prefix_length | 3 | uimsbf |
| international_area_code_length | 2 | uimsbf |
| operator_code_length | 1 | bslbf |
| reserved for future use | 3 | uimsbf |
| national_area_code_length | 4 | uimsbf |
| core_number_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| country_prefix_char | 8 | uimsbf |
| } | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| international_area_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| operator_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| national_area_code_char |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| core_number_char |  |  |
| } |  |  |
| } |  |  |

### Table 101 — Teletext descriptor
_PDF pages 108-108 (§6.2.44)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| teletext_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 24 | bslbf |
| for (i=0;i<N;i++) { | 5 | uimsbf |
| ISO_639_language_code | 3 | uimsbf |
| teletext_type | 8 | uimsbf |
| teletext_magazine_number |  |  |
| teletext_page_number |  |  |
| } |  |  |
| } |  |  |

### Table 102 — Teletext type coding
_PDF pages 108-108 (§6.2.44)_

| teletext_type | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | initial teletext page |
| 0x02 | teletext subtitle page |
| 0x03 | additional information page |
| 0x04 | programme schedule page |
| 0x05 | teletext subtitle page for hearing impaired people |
| 0x06 to 0x1F | reserved for future use |

### Table 103 — Time shifted event descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_shifted_event_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| reference_service_id | 16 | uimsbf |
| reference_event_id |  |  |
| } |  |  |

### Table 104 — Time shifted service descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| time_shifted_service_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 16 | uimsbf |
| reference_service_id |  |  |
| } |  |  |

### Table 105 — Transport stream descriptor
_PDF pages 109-109 (§6.2.46)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| transport_stream_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| byte |  |  |
| } |  |  |
| } |  |  |

### Table 106 — VBI data descriptor
_PDF pages 110-110 (§6.2.47)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| VBI_data_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| data_service_id | 2 | bslbf |
| data_service_descriptor_length | 1 | bslbf |
| if (data_service_id == 0x01 | 5 | uimsbf |
| \|\| data_service_id == 0x02 | 8 | bslbf |
| \|\| data_service_id == 0x04 |  |  |
| \|\| data_service_id == 0x05 |  |  |
| \|\| data_service_id == 0x06 |  |  |
| \|\| data_service_id == 0x07) { |  |  |
| for (j=0;j<N;j++) { |  |  |
| reserved_future_use |  |  |
| field_parity |  |  |
| line_offset |  |  |
| } |  |  |
| } else { |  |  |
| for (j=0;j<N;j++) { |  |  |
| reserved_future_use |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 107 — Data service id coding
_PDF pages 110-110 (§6.2.47)_

| data_service_id | Description |
|---|---|
| 0x00 | reserved for future use |
| 0x01 | EBU teletext (requires additional teletext_descriptor) |
| 0x02 | inverted teletext |
| 0x03 | reserved for future use |
| 0x04 | Video Programme System (VPS) |
| 0x05 | Wide Screen Signalling (WSS) |
| 0x06 | closed captioning |
| 0x07 | monochrome 4:2:2 samples |
| 0x08 to 0xEF | reserved for future use |
| 0xF0 to 0xFF | user defined |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2, PDF pages 3-3. 94 tables / 826 rows reproduced verbatim._
