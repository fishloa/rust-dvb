# Supplementary Audio (extension sub-tag 0x06)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.11
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x06-supplementary_audio.rs`
**Rust struct:** `SupplementaryAudioDescriptor<'a>`

## Tables

### Table 153 — Supplementary audio descriptor
_PDF pages 141-141 (§6.4.11)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| supplementary_audio_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 1 | uimsbf |
| mix_type | 5 | uimsbf |
| editorial_classification | 1 | bslbf |
| reserved_future_use | 1 | uimsbf |
| language_code_present | 24 | bslbf |
| if (language_code_present == 0b1) { | 8 | uimsbf |
| ISO_639_language_code |  |  |
| } |  |  |
| for (i=0;i<N;i++) { |  |  |
| private_data_byte |  |  |
| } |  |  |
| } |  |  |

### Table 154 — Mix type coding
_PDF pages 141-141 (§6.4.11)_

| mix_type (see note) | Description |
|---|---|
| 0 | the audio stream is a dependent stream and is intended to be mixed or combined |
|  | with a separate complete and independent audio stream by the receiver |
| 1 | the audio stream is a complete and independent stream |
| NOTE: Restrictions on valid combinations of audio_type, mix_type, and editorial_classification are |  |
| given in clause J.4. |  |

### Table 155 — Editorial classification coding
_PDF pages 141-141 (§6.4.11)_

| editorial_classification | Description |
|---|---|
| (see note) |  |
| 0x00 | main audio (contains all of the main audio components and can be |
|  | presented on its own or mixed with a supplementary audio stream) |
|  | This classification shall not be used for broadcast-mix audio (see |
|  | clause J.3) e.g. audio streams that are premixed with visual impaired or |
|  | hearing impaired audio. |
| 0x01 | audio description for the visually impaired (contains a spoken description |
|  | of the visual content of the service) |
| 0x02 | clean audio for the hearing impaired |
| 0x03 | spoken subtitles for the visually impaired |
| 0x04 | dependent parametric data stream (not embedded) |
| 0x05 to 0x16 | reserved for future use |
| 0x17 | unspecific supplementary audio for the general audience |
| 0x18 to 0x1F | user defined |
| NOTE: Restrictions on valid combinations of audio_type, mix_type, and editorial_classification are |  |
| given in clause J.4. |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.11, PDF pages 5-24. 3 tables / 16 rows reproduced verbatim._
