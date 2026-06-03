# SI Tables Reference

**Source:** ETSI EN 300 468 v1.19.1 §5.2 + ISO/IEC 13818-1

## MPEG-2 PSI Tables

| Table | table_id | PID | File |
|---|---|---|---|
| PAT — Program Association | 0x00 | 0x0000 | [pat.md](pat.md) |
| CAT — Conditional Access | 0x01 | 0x0001 | [cat.md](cat.md) |
| PMT — Program Map | 0x02 | per-programme | [pmt.md](pmt.md) |

## DVB SI Tables

| Table | table_id | PID | File |
|---|---|---|---|
| NIT — Network Information (actual) | 0x40 | 0x0010 | [nit.md](nit.md) |
| NIT — Network Information (other) | 0x41 | 0x0010 | [nit.md](nit.md) |
| BAT — Bouquet Association | 0x4A | 0x0011 | [bat.md](bat.md) |
| SDT — Service Description (actual) | 0x42 | 0x0011 | [sdt.md](sdt.md) |
| SDT — Service Description (other) | 0x46 | 0x0011 | [sdt.md](sdt.md) |
| EIT — Event Information (p/f actual) | 0x4E | 0x0012 | [eit.md](eit.md) |
| EIT — Event Information (p/f other) | 0x4F | 0x0012 | [eit.md](eit.md) |
| EIT — Schedule (actual, 16 sections) | 0x50–0x5F | 0x0012 | [eit.md](eit.md) |
| EIT — Schedule (other, 16 sections) | 0x60–0x6F | 0x0012 | [eit.md](eit.md) |
| TDT — Time and Date | 0x70 | 0x0014 | [tdt.md](tdt.md) |
| RST — Running Status | 0x71 | 0x0013 | [rst.md](rst.md) |
| ST — Stuffing | 0x72 | multi | [st.md](st.md) |
| TOT — Time Offset | 0x73 | 0x0014 | [tot.md](tot.md) |
| DIT — Discontinuity Information | 0x7E | 0x001E | [dit.md](dit.md) |
| SIT — Selection Information | 0x7F | 0x001F | [sit.md](sit.md) |
| SAT — Satellite Access (family) | 0x4D | 0x001B | [sat/README.md](sat/README.md) |

## SAT Family (§5.2.11)

| Sub-table | File |
|---|---|
| Satellite Position v2 | [sat/position_v2.md](sat/position_v2.md) |
| Cell Fragment | [sat/cell_fragment.md](sat/cell_fragment.md) |
| Time Association | [sat/time_association.md](sat/time_association.md) |
| Beamhopping Time Plan | [sat/beamhopping_time_plan.md](sat/beamhopping_time_plan.md) |
| Satellite Position v3 | [sat/position_v3.md](sat/position_v3.md) |
