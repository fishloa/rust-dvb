# DVB SI Reference — ETSI EN 300 468 v1.19.1

Structured markdown reference extracted from ETSI EN 300 468 v1.19.1 (February 2025).
Consumed by the local Qwen3.6-35B-A3B model (via opencode) during implementation of
the `crates/dvb_si/` crate.

## Structure

```
docs/dvb_si/
├── INDEX.md                    Top-level navigation
├── overview.md                 §1-§4: scope, PID table, table_id allocation, bit order
├── glossary.md                 Terms and abbreviations
├── tables/
│   ├── README.md               Table summary with table_id and PID
│   ├── pat.md, pmt.md, cat.md  MPEG-2 PSI tables
│   ├── nit.md, bat.md, sdt.md  DVB network/bouquet/service tables
│   ├── eit.md                  Event Information Table (EPG)
│   ├── tdt.md, tot.md          Time tables
│   ├── rst.md, st.md           Running status and stuffing
│   ├── dit.md, sit.md          Discontinuity and selection info
│   └── sat/                    §5.2.11 Satellite Access Tables (v1.19 addition)
├── descriptors/
│   ├── INDEX.md                Tag → file lookup (numerical + alphabetical)
│   ├── 0x02-*.md … 0x7E-*.md  One file per descriptor tag
│   └── 0x7F-extension/
│       ├── README.md           Sub-tag dispatcher + enum
│       └── 0x00-*.md …        One file per extension sub-tag
├── text/
│   ├── annex_a.md              Character set selection mechanism
│   └── charsets/               One file per character table (16 total)
├── crc/
│   └── annex_c.md              CRC-32 polynomial and algorithm
└── annexes/
    ├── annex_d.md              AC-3/E-AC-3 SI implementation
    ├── annex_g.md              DTS audio SI
    ├── annex_h.md              AAC audio SI
    ├── annex_i.md              service_type field values
    └── annex_j.md, k.md, l.md  Additional normative annexes
```

## Per-entity file format

Every file follows a strict template:

1. **Purpose** — 2-5 sentences: what this entity is, when it appears
2. **Syntax** — markdown table reproducing the spec's bit-layout table
3. **Semantics** — field-by-field meanings
4. **Value enumerations** — any value tables (enum coding)
5. **Parser requirements** — bit-level considerations, error conditions
6. **Byte example** — hex dump annotated field-by-field (or TODO)
7. **Cross-references** — which tables carry this descriptor; related entities

Files marked `<!-- HAND-POLISHED -->` on line 1 are protected from regeneration.

## Regenerating from a new spec version

1. Run `pdftotext -layout <new_pdf> /tmp/dvb-extract/full.txt`
2. Run `python3 tools/dvb-si-extract/extract.py --input /tmp/dvb-extract/full.txt --output docs/dvb_si`
3. The script skips files marked `<!-- HAND-POLISHED -->`
4. Diff the new drafts against the old ones: `git diff docs/dvb_si/`
5. Manually review and update the hand-polished files for any spec changes

## Hand-polish priority list

Files that need the most manual attention (complex tables, many subtleties):

1. `tables/eit.md` — EIT is our most-consumed table; has complex event loop + running_status enum
2. `tables/nit.md` — descriptor loops for network and transport stream loops
3. `tables/sdt.md` — SDT service loop with multiple descriptor contexts
4. `descriptors/0x50-component.md` — Table 26 is huge (stream_content × component_type)
5. `descriptors/0x54-content.md` — genre nibble table
6. `descriptors/0x4A-linkage.md` — has 4 sub-types (mobile handover, event, extended event)
7. `descriptors/0x43-satellite_delivery_system.md` — BCD frequency encoding, polarization bits
8. `descriptors/0x7F-extension/0x04-t2_delivery_system.md` — complex; many sub-fields
9. `text/annex_a.md` — character set selection rules
10. `crc/annex_c.md` — CRC algorithm + lookup table
