# DVB SI Reference Index

**Source:** ETSI EN 300 468 v1.19.1 (February 2025)
**Generated from:** `tools/dvb-si-extract/extract.py`
**Target crate:** `crates/dvb_si/`

This reference is consumed by the local Qwen3.6-35B-A3B model via opencode.
Every entity has its own file; each file is self-sufficient (no cross-file lookups needed for parsing).

## Tables

| Table | table_id | File |
|---|---|---|
| PAT — Program Association | 0x00 | [tables/pat.md](tables/pat.md) |
| CAT — Conditional Access | 0x01 | [tables/cat.md](tables/cat.md) |
| PMT — Program Map | 0x02 | [tables/pmt.md](tables/pmt.md) |
| NIT — Network Information | 0x40/0x41 | [tables/nit.md](tables/nit.md) |
| BAT — Bouquet Association | 0x4A | [tables/bat.md](tables/bat.md) |
| SDT — Service Description | 0x42/0x46 | [tables/sdt.md](tables/sdt.md) |
| EIT — Event Information | 0x4E–0x6F | [tables/eit.md](tables/eit.md) |
| TDT — Time and Date | 0x70 | [tables/tdt.md](tables/tdt.md) |
| TOT — Time Offset | 0x73 | [tables/tot.md](tables/tot.md) |
| RST — Running Status | 0x71 | [tables/rst.md](tables/rst.md) |
| ST — Stuffing | 0x72 | [tables/st.md](tables/st.md) |
| DIT — Discontinuity Information | 0x7E | [tables/dit.md](tables/dit.md) |
| SIT — Selection Information | 0x7F | [tables/sit.md](tables/sit.md) |
| SAT — Satellite Access (family) | 0x4D | [tables/sat/README.md](tables/sat/README.md) |

## Descriptors

See [descriptors/INDEX.md](descriptors/INDEX.md) for the full tag-to-file lookup.

### MPEG-2 descriptors (used in DVB PMTs)

| Tag | Descriptor | File |
|---|---|---|
| 0x02 | video_stream | [descriptors/0x02-video_stream.md](descriptors/0x02-video_stream.md) |
| 0x03 | audio_stream | [descriptors/0x03-audio_stream.md](descriptors/0x03-audio_stream.md) |
| 0x05 | registration | [descriptors/0x05-registration.md](descriptors/0x05-registration.md) |
| 0x09 | CA | [descriptors/0x09-ca.md](descriptors/0x09-ca.md) |
| 0x0A | ISO_639_language | [descriptors/0x0A-iso_639_language.md](descriptors/0x0A-iso_639_language.md) |

### DVB descriptors (0x40–0x7F)

| Tag | Descriptor | File |
|---|---|---|
| 0x40 | network_name | [descriptors/0x40-network_name.md](descriptors/0x40-network_name.md) |
| 0x41 | service_list | [descriptors/0x41-service_list.md](descriptors/0x41-service_list.md) |
| 0x43 | satellite_delivery_system | [descriptors/0x43-satellite_delivery_system.md](descriptors/0x43-satellite_delivery_system.md) |
| 0x44 | cable_delivery_system | [descriptors/0x44-cable_delivery_system.md](descriptors/0x44-cable_delivery_system.md) |
| 0x48 | service | [descriptors/0x48-service.md](descriptors/0x48-service.md) |
| 0x4A | linkage | [descriptors/0x4A-linkage.md](descriptors/0x4A-linkage.md) |
| 0x4D | short_event | [descriptors/0x4D-short_event.md](descriptors/0x4D-short_event.md) |
| 0x4E | extended_event | [descriptors/0x4E-extended_event.md](descriptors/0x4E-extended_event.md) |
| 0x50 | component | [descriptors/0x50-component.md](descriptors/0x50-component.md) |
| 0x54 | content | [descriptors/0x54-content.md](descriptors/0x54-content.md) |
| 0x55 | parental_rating | [descriptors/0x55-parental_rating.md](descriptors/0x55-parental_rating.md) |
| 0x58 | local_time_offset | [descriptors/0x58-local_time_offset.md](descriptors/0x58-local_time_offset.md) |
| 0x59 | subtitling | [descriptors/0x59-subtitling.md](descriptors/0x59-subtitling.md) |
| 0x5A | terrestrial_delivery_system | [descriptors/0x5A-terrestrial_delivery_system.md](descriptors/0x5A-terrestrial_delivery_system.md) |
| 0x5F | private_data_specifier | [descriptors/0x5F-private_data_specifier.md](descriptors/0x5F-private_data_specifier.md) |
| 0x6A | AC-3 | [descriptors/0x6A-ac3.md](descriptors/0x6A-ac3.md) |
| 0x7A | enhanced_AC-3 | [descriptors/0x7A-enhanced_ac3.md](descriptors/0x7A-enhanced_ac3.md) |
| 0x7C | AAC | [descriptors/0x7C-aac.md](descriptors/0x7C-aac.md) |
| 0x7E | FTA_content_management | [descriptors/0x7E-fta_content_management.md](descriptors/0x7E-fta_content_management.md) |
| 0x7F | extension | [descriptors/0x7F-extension/README.md](descriptors/0x7F-extension/README.md) |

## Extension descriptors (§6.4)

| Sub-tag | Descriptor | File |
|---|---|---|
| 0x00 | image_icon | [descriptors/0x7F-extension/0x00-image_icon.md](descriptors/0x7F-extension/0x00-image_icon.md) |
| 0x04 | T2_delivery_system | [descriptors/0x7F-extension/0x04-t2_delivery_system.md](descriptors/0x7F-extension/0x04-t2_delivery_system.md) |
| 0x06 | supplementary_audio | [descriptors/0x7F-extension/0x06-supplementary_audio.md](descriptors/0x7F-extension/0x06-supplementary_audio.md) |
| 0x0D | C2_delivery_system | [descriptors/0x7F-extension/0x0D-c2_delivery_system.md](descriptors/0x7F-extension/0x0D-c2_delivery_system.md) |
| 0x11 | T2MI | [descriptors/0x7F-extension/0x11-t2mi.md](descriptors/0x7F-extension/0x11-t2mi.md) |
| 0x13 | URI_linkage | [descriptors/0x7F-extension/0x13-uri_linkage.md](descriptors/0x7F-extension/0x13-uri_linkage.md) |
| 0x15 | AC-4 | [descriptors/0x7F-extension/0x15-ac4.md](descriptors/0x7F-extension/0x15-ac4.md) |
| 0x17 | S2X_satellite_delivery_system | [descriptors/0x7F-extension/0x17-s2x_satellite_delivery_system.md](descriptors/0x7F-extension/0x17-s2x_satellite_delivery_system.md) |
| 0x18 | audio_preselection | [descriptors/0x7F-extension/0x18-audio_preselection.md](descriptors/0x7F-extension/0x18-audio_preselection.md) |

## Text encoding

- [text/annex_a.md](text/annex_a.md) — Character set selection mechanism
- [text/charsets/](text/charsets/) — One file per character table

## CRC and utilities

- [crc/annex_c.md](crc/annex_c.md) — MPEG-2 CRC-32 polynomial

## Annexes

- [annexes/annex_d.md](annexes/annex_d.md) — AC-3 / Enhanced AC-3 SI
- [annexes/annex_g.md](annexes/annex_g.md) — DTS audio SI
- [annexes/annex_h.md](annexes/annex_h.md) — AAC audio SI
- [annexes/annex_i.md](annexes/annex_i.md) — service_type field values

## Other files

- [overview.md](overview.md) — §1–§4 scope, PID table, table_id allocation
- [glossary.md](glossary.md) — Terms and abbreviations
- [README.md](README.md) — How this reference is structured and how to regenerate it
