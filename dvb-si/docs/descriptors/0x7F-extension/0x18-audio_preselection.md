# Audio Preselection (extension sub-tag 0x18)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.1
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x18-audio_preselection.rs`
**Rust struct:** `AudioPreselectionDescriptor<'a>`

## Tables

### Table 110 — Audio preselection descriptor
_PDF pages 114-114 (§6.4.1)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| audio_preselection_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 5 | uimsbf |
| num_preselections | 3 | bslbf |
| reserved_zero_future_use | 5 | uimsbf |
| for (i=0;i<N;i++) { | 3 | uimsbf |
| preselection_id | 1 | bslbf |
| audio_rendering_indication | 1 | bslbf |
| audio_description | 1 | bslbf |
| spoken_subtitles | 1 | bslbf |
| dialogue_enhancement | 1 | bslbf |
| interactivity_enabled | 1 | bslbf |
| language_code_present | 1 | bslbf |
| text_label_present | 1 | bslbf |
| multi_stream_info_present | 24 | bslbf |
| future_extension | 8 | uimsbf |
| if (language_code_present == 0b1) { | 3 | uimsbf |
| ISO_639_language_code | 5 | bslbf |
| } | 8 | uimsbf |
| if (text_label_present == 0b1) { | 3 | bslbf |
| message_id | 5 | uimsbf |
| } | 8 | uimsbf |
| if (multi_stream_info_present == 0b1) { |  |  |
| num_aux_components |  |  |
| reserved_zero_future_use |  |  |
| for (j=0;j<N;j++) { |  |  |
| component_tag |  |  |
| } |  |  |
| } |  |  |
| if (future_extension == 0b1) { |  |  |
| reserved_zero_future_use |  |  |
| future_extension_length |  |  |
| for (j=0;j<N;j++) { |  |  |
| future_extension_byte |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 111 — Audio rendering indication coding
_PDF pages 115-115 (§6.4.1)_

| audio_rendering_indication | Description |
|---|---|
| 0 | no preference given for the reproduction channel layout |
| 1 | preferred reproduction channel layout is stereo |
| 2 | preferred reproduction channel layout is two-dimensional (e.g. 5.1 multi-channel) |
| 3 | preferred reproduction channel layout is three-dimensional |
| 4 | content is pre-rendered for consumption with headphones |
| 5 to 7 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.1, PDF pages 4-4. 2 tables / 9 rows reproduced verbatim._
