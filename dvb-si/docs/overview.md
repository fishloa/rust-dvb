# Overview — ETSI EN 300 468 v1.19.1

**Spec:** ETSI EN 300 468 v1.19.1 §1–§4
**Reference:** `docs/dvb_si/`

## Scope (§1)

ETSI EN 300 468 specifies the Service Information (SI) data for DVB (Digital Video Broadcasting)
bitstreams. SI data enables:

- User-assisted service selection (channel listings, EPG)
- Automatic IRD (Integrated Receiver Decoder) configuration
- Automatic tuning to correct transponder parameters
- Event-based information (EPG/scheduling data)

The spec complements MPEG-2 PSI (Program Specific Information as defined in ISO/IEC 13818-1)
with DVB-specific tables and descriptors. Presentation of SI to users is intentionally unspecified.

## Key PIDs (§5.1.3 Table 1)

| PID | Table(s) |
|---|---|
| 0x0000 | PAT |
| 0x0001 | CAT |
| 0x0002 | TSDT |
| 0x0010 | NIT, ST |
| 0x0011 | SDT, BAT, ST |
| 0x0012 | EIT, ST |
| 0x0013 | RST, ST |
| 0x0014 | TDT, TOT, ST |
| 0x0015 | network synchronization |
| 0x001B | SAT |
| 0x001E | DIT |
| 0x001F | SIT |
| 0x1FFF | Null packets |

## Table ID allocation (§5.1.3 Table 2)

| table_id | Table |
|---|---|
| 0x00 | PAT (program_association_section) |
| 0x01 | CAT (conditional_access_section) |
| 0x02 | PMT (program_map_section) |
| 0x03 | TSDT |
| 0x04–0x3F | reserved |
| 0x40 | NIT actual |
| 0x41 | NIT other |
| 0x42 | SDT actual |
| 0x46 | SDT other |
| 0x4A | BAT |
| 0x4D | SAT |
| 0x4E | EIT p/f actual |
| 0x4F | EIT p/f other |
| 0x50–0x5F | EIT schedule actual |
| 0x60–0x6F | EIT schedule other |
| 0x70 | TDT |
| 0x71 | RST |
| 0x72 | ST (Stuffing) |
| 0x73 | TOT |
| 0x7E | DIT |
| 0x7F | SIT |
| 0x80–0xFE | user defined |

## Section size limits

- Standard SI sections: max 1 024 bytes
- EIT sections: max 4 096 bytes
- SAT sections: max 4 096 bytes

## Transmission rules (§5.1.4)

- Minimum re-transmission interval: 25 ms (between last byte of one section and first byte of the next
  with same PID, table_id, table_id_extension)
- Applies to transport streams up to 100 Mbit/s
- SAT has different repetition rules (see ETSI TS 101 211)

## Scrambling (§5.1.5)

All tables shall NOT be scrambled, with the exception of EIT schedule. If EIT schedule is scrambled,
it is identified via service_id 0xFFFF in PSI.

## Bit ordering (§5.1.6)

Fields are big-endian (`uimsbf` = unsigned integer, most significant bit first; `bslbf` = bit string,
left bit first). Fields are transmitted top-to-bottom as listed in syntax tables, MSB first within each field.

## Version numbering

`version_number` (5 bits) increments by 1 on change, wrapping 31→0. `current_next_indicator=1`
means the section is currently applicable; =0 means it is the next version.

## Cross-references

- Tables: `docs/dvb_si/tables/`
- Descriptors: `docs/dvb_si/descriptors/`
- Text encoding: `docs/dvb_si/text/annex_a.md`
- CRC-32: `docs/dvb_si/crc/annex_c.md`
- Glossary: `docs/dvb_si/glossary.md`
