# Network Change Notify (extension sub-tag 0x07)

**Spec:** ETSI EN 300 468 v1.19.1 §6.4.9
**Parser file:** `crates/dvb_si/src/descriptors/extension/0x07-network_change_notify.rs`
**Rust struct:** `NetworkChangeNotifyDescriptor<'a>`

## Tables

### Table 148 — Message descriptor
_PDF pages 137-137 (§6.4.9)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| message_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 8 | uimsbf |
| message_id | 24 | bslbf |
| ISO_639_language_code | 8 | uimsbf |
| for (i=0;i<N;i++) { |  |  |
| text_char |  |  |
| } |  |  |
| } |  |  |

### Table 149 — Network change notify descriptor
_PDF pages 138-138 (§6.4.9)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| network_change_notify_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 8 | uimsbf |
| descriptor_tag_extension | 16 | uimsbf |
| for (i=0;i<N;i++) { | 8 | uimsbf |
| cell_id | 8 | uimsbf |
| loop_length | 8 | uimsbf |
| for (j=0;j<N;j++) { | 40 | bslbf |
| network_change_id | 24 | uimsbf |
| network_change_version | 3 | uimsbf |
| start_time_of_change | 1 | bslbf |
| change_duration | 4 | uimsbf |
| receiver_category | 8 | uimsbf |
| invariant_ts_present | 16 | uimsbf |
| change_type | 16 | uimsbf |
| message_id |  |  |
| if (invariant_ts_present == 0b1) { |  |  |
| invariant_ts_tsid |  |  |
| invariant_ts_onid |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 150 — Receiver category coding
_PDF pages 138-138 (§6.4.9)_

| receiver_category | Description |
|---|---|
| 0 | all receivers |
| 1 | DVB-T2, or DVB-S2, or DVB-C2 capable receivers only |
| 2 to 7 | reserved for future use |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.4.9, PDF pages 5-24. 3 tables / 8 rows reproduced verbatim._
