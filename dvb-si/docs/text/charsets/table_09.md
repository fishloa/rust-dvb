# Character Table 09 — Latin No. 7 (ISO/IEC 8859-13)

**Spec:** ETSI EN 300 468 v1.19.1 Annex A, Table A.3
**Implementation:** `crates/dvb_si/src/text/charsets.rs`

## Description

Latin alphabet No. 7 (Baltic Rim). Prefix byte 0x09.

## Prefix bytes

| Table | Prefix |
|---|---|
| Table 09 | `0x09` (or `0x10 0x00 0x09` for ISO 8859 variant) |

## Usage in DVB SI

Text fields in DVB SI that use this character table begin with prefix byte `0x09`.
If no prefix is present, table 00 (default Latin) is assumed.

## Implementation notes

- For tables 0x00–0x0B: single-byte encoding (256 code points)
- For table 0x11 (UCS-2): 2 bytes per character, big-endian
- For table 0x15 (UTF-8): variable-length encoding
- Decoder must handle control codes 0x80–0x9F (emphasis, CR/LF)

## Cross-references

- Parent: `docs/dvb_si/text/annex_a.md`
- Implementation: `crates/dvb_si/src/text/charsets.rs`
