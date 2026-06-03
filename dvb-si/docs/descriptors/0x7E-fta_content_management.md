# Fta Content Management (tag 0x7E)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.18
**Parser file:** `crates/dvb_si/src/descriptors/0x7E-fta_content_management.rs`
**Rust struct:** `FtaContentManagementDescriptor<'a>`

## Tables

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

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.18, PDF pages 4-4. 5 tables / 16 rows reproduced verbatim._
