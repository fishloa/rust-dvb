# Annex A — Coding of Text Characters

**Spec:** ETSI EN 300 468 v1.19.1 Annex A (normative)
**Implementation:** `crates/dvb_si/src/text/mod.rs` + `crates/dvb_si/src/text/charsets.rs`

## Purpose

Annex A defines how text strings in DVB SI (service names, event titles, descriptions, etc.)
are encoded. Text can be in one of 17 character sets selected by optional prefix bytes. If no
prefix is present, the default Latin table (ISO 6937) is assumed. Control codes in 0x80–0x9F
provide emphasis on/off and CR/LF.

## Character table selection

If the first byte of a text field is in 0x20–0xFF, the default Latin table (table 00, ISO 6937) applies
for the entire string.

If the first byte is 0x01–0x0B or 0x10–0x14, it is a character table selector:

| First byte | Character table | Standard |
|---|---|---|
| 0x01 | Latin/Cyrillic | ISO/IEC 8859-5 |
| 0x02 | Latin/Arabic | ISO/IEC 8859-6 |
| 0x03 | Latin/Greek | ISO/IEC 8859-7 |
| 0x04 | Latin/Hebrew | ISO/IEC 8859-8 |
| 0x05 | Latin No. 5 | ISO/IEC 8859-9 |
| 0x06 | Latin No. 6 | ISO/IEC 8859-10 |
| 0x07 | Latin/Thai | ISO/IEC 8859-11 |
| 0x08 | reserved | — |
| 0x09 | Latin No. 7 | ISO/IEC 8859-13 |
| 0x0A | Latin No. 8 (Celtic) | ISO/IEC 8859-14 |
| 0x0B | Latin No. 9 | ISO/IEC 8859-15 |
| 0x0C–0x0F | reserved | — |
| 0x10 | dynamic ISO/IEC 8859 selection | see Table A.4 (two-byte selector) |
| 0x11 | Unicode BMP (UCS-2 big-endian) | ISO/IEC 10646 |
| 0x12 | Korean | KS X 1001-2014 |
| 0x13 | Simplified Chinese | GB-2312-1980 |
| 0x14 | Traditional Chinese (Big5) | ISO/IEC 10646 subset |
| 0x15 | Unicode UTF-8 | ISO/IEC 10646 |

When byte 0x10 appears, the next two bytes form a 16-bit code indicating which ISO/IEC 8859-X table
to use (see Table A.4).

## Control codes (single-byte tables)

| Code | UTF-8 equivalent | Meaning |
|---|---|---|
| 0x80–0x85 | 0xC2 0x80–0xC2 0x85 | reserved |
| 0x86 | 0xC2 0x86 | character emphasis ON |
| 0x87 | 0xC2 0x87 | character emphasis OFF |
| 0x88–0x89 | 0xC2 0x88–0xC2 0x89 | reserved |
| 0x8A | 0xC2 0x8A | CR/LF |
| 0x8B–0x9F | 0xC2 0x8B–0xC2 0x9F | user defined |

## Control codes (two-byte tables)

Control codes are placed in Unicode Private Use Area 0xE080–0xE09F (UTF-8: 0xEE 0x82 0x80 –
0xEE 0x82 0x9F). Same semantics as single-byte table codes.

## Parser requirements

1. Read first byte: if 0x20–0xFF → default table (Latin/ISO 6937); if < 0x20 → it is a selector.
2. If selector byte is 0x10: consume next 2 bytes as ISO 8859 sub-table ID.
3. If selector byte is 0x11 or 0x15: all subsequent bytes are UCS-2 BE or UTF-8.
4. Decode control codes 0x86/0x87 as emphasis markers (consumers may strip or honour them).
5. 0x8A → newline.
6. Return `Cow<str>`: borrow for UTF-8 already-valid slices, allocate otherwise.

## Cross-references

- Used by: every text field in NIT, BAT, SDT, EIT, TOT descriptors
- Character tables: `docs/dvb_si/text/charsets/`
- Implementation notes: `crates/dvb_si/src/text/mod.rs`
