# Service Descriptor (tag 0x48)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.33
**Parser file:** `crates/dvb_si/src/descriptors/service.rs`
**Rust struct:** `ServiceDescriptor<'a>`

## Tables

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

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.33, PDF pages 4-4. 3 tables / 51 rows reproduced verbatim._
