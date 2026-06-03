# Program Map Table (table_id 0x02)

**Spec:** ETSI EN 300 468 v1.19.1 ISO/IEC 13818-1 §2.4.4.8
**Parser file:** `crates/dvb_si/src/tables/pmt.rs`
**Rust struct:** `Pmt`

## Purpose

Semantics for the network information section: table_id: See table 2. section_syntax_indicator: This 1-bit field shall be set to 0b1.

## Syntax

| Field | Bits | Format | Notes |
|---|---|---|---|
| table_id | 8 | uimsbf | |
| section_syntax_indicator | 1 | bslbf | |
| reserved_future_use | 1 | bslbf | |
| reserved | 2 | bslbf | |
| section_length | 12 | uimsbf | |
| network_id | 16 | uimsbf | |
| reserved | 2 | bslbf | |
| version_number | 5 | uimsbf | |
| current_next_indicator | 1 | bslbf | |
| section_number | 8 | uimsbf | |
| last_section_number | 8 | uimsbf | |
| reserved_future_use | 4 | bslbf | |
| network_descriptors_length | 12 | uimsbf | |
| *for (i=0;i<N;i++) {* | | | loop/conditional |

## Semantics

table_id: See table 2.

section_syntax_indicator: This 1-bit field shall be set to 0b1.

ETSI

30                         ETSI EN 300 468 V1.19.1 (2025-02)

section_length: This is a 12-bit field, the first two bits of which shall be 0b00. It specifies the number of bytes of the

section, starting immediately following the section_length field and including the Cyclic Redundancy Check (CRC).

The value in the section_length field shall not exceed 1 021 so that the entire section has a maximum length of

1 024 bytes.

network_id: This is a 16-bit field which serves as a label to identify the delivery system, about which the NIT informs,

from any other delivery system. It shall be coded according to ETSI TS 101 162 [15].

version_number: This 5-bit field is the version number of the sub_table. The version_number shall be incremented by 1

when a change in the information carried within the sub_table occurs. When it reaches value 31, it wraps around to 0.

When the current_next_indicator is set to 0b1, then the version_number shall be that of the currently applicable

## Value enumerations

| Value | Description |
|---|---|
| `0x02` | program_map_section |
| `0x03` | transport_stream_description_section |
| `0x04 to 0x3F` | reserved |
| `0x40` | network_information_section - actual network |
| `0x41` | network_information_section - other network |
| `0x42` | service_description_section - actual DVB transport stream |
| `0x43 to 0x45` | reserved for future use |
| `0x46` | service_description_section - other DVB transport stream |
| `0x47 to 0x49` | reserved for future use |
| `0x4A` | bouquet_association_section |
| `0x4B` | update notification table section (ETSI TS 102 006 [20]) |
| `0x4C` | IP/MAC_notification_section (ETSI EN 301 192 [3] - see note 2) |
| `0x4D` | satellite_access_section |
| `0x4E` | event_information_section - actual DVB transport stream, present/following |
| `0x4F` | event_information_section - other DVB transport stream, present/following |
| `0x50 to 0x5F` | event_information_section - actual DVB transport stream, schedule |
| `0x60 to 0x6F` | event_information_section - other DVB transport stream, schedule |
| `0x70` | time_date_section |
| `0x71` | running_status_section |
| `0x72` | stuffing_section |
| `0x73` | time_offset_section |
| `0x74` | application information section (ETSI TS 102 812 [26]) |
| `0x75` | container section (ETSI TS 102 323 [21]) |
| `0x76` | related content section (ETSI TS 102 323 [21]) |
| `0x77` | content identifier section (ETSI TS 102 323 [21]) |
| `0x78` | MPE-FEC section (ETSI EN 301 192 [3]) |
| `0x79` | resolution provider notification section (ETSI TS 102 323 [21]) |
| `0x7A` | MPE-IFEC section (ETSI TS 102 772 [23]) |
| `0x7B` | protection message section (ETSI TS 102 809 [25]) |
| `0x7C` | downloadable font info section (ETSI EN 303 560 [12] - see note 2) |
| `0x7D` | reserved for future use |
| `0x7E` | discontinuity_information_section |
| `0x7F` | selection_information_section |
| `0x80 to 0xFE` | user defined |
| `0xFF` | reserved (see note 1) |

## Parser requirements

- CRC-32 validation (Annex C polynomial) on the entire section
- `section_syntax_indicator` must be 1 (except ST and DIT)
- `section_length` limits parsing to that many bytes (max 1 024 for most tables; 4 096 for EIT and SAT)
- Reassembly: collect all sections with same `(table_id, table_id_extension, version_number)` before parsing

## Byte example

No byte example in spec — TODO: fill from a real capture.

## Cross-references

- PID: per-programme (signalled in PAT)
- Related tables: _see spec §5.1.3 Table 2_
- Related spec sections: ISO/IEC 13818-1 §2.4.4.8
