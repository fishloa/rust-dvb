# Component Descriptor (tag 0x50)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.8
**Parser file:** `crates/dvb_si/src/descriptors/component.rs`
**Rust struct:** `ComponentDescriptor<'a>`

## Spec text

### §6.2.8 Component descriptor ................................................................................................................................. 59

## Tables

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

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.8, PDF pages 4-4. 2 tables / 198 rows reproduced verbatim._
