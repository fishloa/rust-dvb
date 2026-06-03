# SAT — Satellite Access Table (table_id 0x4D)

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.11
**table_id:** 0x4D
**PID:** 0x0010 (SAT is carried on the NIT PID per §5.1.3).
**Parser file:** `dvb-si/src/tables/sat.rs`
**Rust struct:** `Sat`

> **Authoritative, hand-transcribed from the canonical PDF**
> (`specs/etsi_en_300_468_v01.19.01_dvb_si.pdf`, pp. 40-52). This file replaces
> the earlier `sat/` per-subtable extraction, whose bit-width columns were
> misaligned by `pdfplumber` (e.g. it reported satellite_table_id as 10 bits /
> table_count as 2 bits — the correct widths are 6 / 10).

The SAT is a **family**: a common `satellite_access_section()` header carries a
6-bit `satellite_table_id` discriminant selecting one of five body structures.

## Table 11a — Satellite access section (§5.2.11.1)

| Syntax | No. of bits | Identifier |
|---|---|---|
| `satellite_access_section() {` |  |  |
| &nbsp;&nbsp;table_id | 8 | uimsbf |
| &nbsp;&nbsp;section_syntax_indicator | 1 | bslbf |
| &nbsp;&nbsp;private_indicator | 1 | bslbf |
| &nbsp;&nbsp;reserved | 2 | bslbf |
| &nbsp;&nbsp;section_length | 12 | uimsbf |
| &nbsp;&nbsp;satellite_table_id | 6 | uimsbf |
| &nbsp;&nbsp;table_count | 10 | uimsbf |
| &nbsp;&nbsp;reserved | 2 | bslbf |
| &nbsp;&nbsp;version_number | 5 | uimsbf |
| &nbsp;&nbsp;current_next_indicator | 1 | bslbf |
| &nbsp;&nbsp;section_number | 8 | uimsbf |
| &nbsp;&nbsp;last_section_number | 8 | uimsbf |
| &nbsp;&nbsp;reserved_zero_future_use | 8 | bslbf |
| &nbsp;&nbsp;`if (satellite_table_id == 0) { satellite_position_v2_info() }` |  |  |
| &nbsp;&nbsp;`else if (satellite_table_id == 1) { cell_fragment_info() }` |  |  |
| &nbsp;&nbsp;`else if (satellite_table_id == 2) { time_association_info() }` |  |  |
| &nbsp;&nbsp;`else if (satellite_table_id == 3) { beamhopping_time_plan_info() }` |  |  |
| &nbsp;&nbsp;`else if (satellite_table_id == 4) { satellite_position_v3_info() }` |  |  |
| &nbsp;&nbsp;`else { for (i=0;i<N;i++) { reserved_zero_future_use` | 8 | bslbf |
| &nbsp;&nbsp;`} }` |  |  |
| &nbsp;&nbsp;CRC_32 | 32 | rpchof |
| `}` |  |  |

**table_count** (10 bits) distinguishes sub_tables: for Position it is the 10
MSBs of `satellite_id`; for Beamhopping the 10 MSBs of `beamhopping_time_plan_id`;
for Time Association always 0; for Cell Fragment arbitrary.

### Table 11b — satellite_table_id coding

| satellite_table_id | Body | Clause |
|---|---|---|
| 0 | satellite_position_v2_info | §5.2.11.2 |
| 1 | cell_fragment_info | §5.2.11.3 |
| 2 | time_association_info | §5.2.11.4 |
| 3 | beamhopping_time_plan_info | §5.2.11.5 |
| 4 | satellite_position_v3_info | §5.2.11.6 |
| 5–63 | reserved for future use |  |

## Table 11c — Satellite position v2 info (§5.2.11.2)

| Syntax | No. of bits | Mnemonic |
|---|---|---|
| `satellite_position_v2_info() {` |  |  |
| &nbsp;&nbsp;`for (i=1; i<=N; i++) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;satellite_id | 24 |  |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 7 |  |
| &nbsp;&nbsp;&nbsp;&nbsp;position_system | 1 |  |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (position_system == 0) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;orbital_position | 16 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;west_east_flag | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 7 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (position_system == 1) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;epoch_year | 8 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;day_of_the_year | 16 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;day_fraction | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;mean_motion_first_derivative | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;mean_motion_second_derivative | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;drag_term | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;inclination | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;right_ascension_of_the_ascending_node | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;eccentricity | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;argument_of_perigree | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;mean_anomaly | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;mean_motion | 32 | spfmsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;`}` |  |  |
| `}` |  |  |

`position_system` 0 = geostationary (orbital_position is 4-digit BCD, e.g.
019.2°; west_east_flag 0=west/1=east); 1 = any earth orbit (TLE/SGP4 elements).

## Table 11d — Cell fragment info (§5.2.11.3)

| Syntax | No. of bits | Mnemonic |
|---|---|---|
| `cell_fragment_info() {` |  |  |
| &nbsp;&nbsp;`for (i=1; i<=N; i++) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;cell_fragment_id | 32 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;first_occurence | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;last_occurence | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (first_occurence == 1) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 4 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;center_latitude | 18 | tcimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 5 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;center_longitude | 19 | tcimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;max_distance | 24 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`} else { reserved_zero_future_use` | 4 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;delivery_system_id_loop_count | 10 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`for (j=0; j<delivery_system_id_loop_count; j++) { delivery_system_id` | 32 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;new_delivery_system_id_loop_count | 10 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`for (k=0; k<new_delivery_system_id_loop_count; k++) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;new_delivery_system_id | 32 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;time_of_application_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;time_of_application_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;obsolescent_delivery_system_id_loop_count | 10 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`for (l=0; l<obsolescent_delivery_system_id_loop_count; l++) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;obsolescent_delivery_system_id | 32 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;time_of_obsolescence_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;time_of_obsolescence_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;`}` |  |  |
| `}` |  |  |

`center_latitude`/`center_longitude` are WGS84 in units of 0.001°
(two's-complement). `time_of_application`/`time_of_obsolescence` are NCR
(base = div 300, ext = mod 300) per ETSI EN 301 790.

## Table 11e — Time association info (§5.2.11.4)

| Syntax | No. of bits | Mnemonic |
|---|---|---|
| `time_association_info() {` |  |  |
| &nbsp;&nbsp;association_type | 4 | uimsbf |
| &nbsp;&nbsp;`if (association_type == 1) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;leap59 | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;leap61 | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;pastleap59 | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;pastleap61 | 1 | bslbf |
| &nbsp;&nbsp;`} else { reserved_zero_future_use` | 4 | bslbf |
| &nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;ncr_base | 33 | uimsbf |
| &nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;ncr_ext | 9 | uimsbf |
| &nbsp;&nbsp;association_timestamp_seconds | 64 | uimsbf |
| &nbsp;&nbsp;association_timestamp_nanoseconds | 32 | uimsbf |
| `}` |  |  |

### Table 11f — association_type coding

| association_type | Description |
|---|---|
| 0 | UTC without leap second signalling |
| 1 | UTC with leap second signalling |
| 2–15 | reserved |

## Table 11g — Beamhopping time plan info (§5.2.11.5)

| Syntax | No. of bits | Mnemonic |
|---|---|---|
| `beamhopping_time_plan_info() {` |  |  |
| &nbsp;&nbsp;`for (i=1; i<=N; i++) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;beamhopping_time_plan_id | 32 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 4 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;beamhopping_time_plan_length | 12 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;time_plan_mode | 2 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;time_of_application_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;time_of_application_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;cycle_duration_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;cycle_duration_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (time_plan_mode == 0) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;dwell_duration_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;dwell_duration_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;on_time_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;on_time_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (time_plan_mode == 1) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;bit_map_size | 15 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;current_slot | 15 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`for (j=1; j<=bit_map_size; j++) { slot_transmission_on` | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`for (k=1; k<=J; k++) { padding_bit` | 1 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;`if (time_plan_mode == 2) {` |  |  |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;grid_size_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;grid_size_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;revisit_duration_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;revisit_duration_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;sleep_time_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;sleep_time_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;sleep_duration_base | 33 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;reserved_zero_future_use | 6 | bslbf |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;sleep_duration_ext | 9 | uimsbf |
| &nbsp;&nbsp;&nbsp;&nbsp;`}` |  |  |
| &nbsp;&nbsp;`}` |  |  |
| `}` |  |  |

`padding_bit` count J = 7 − ((bit_map_size − 1) mod 8) (re-align to 8 bits).
All NCR durations are base = div 300, ext = mod 300 (ETSI EN 301 790).

## Table 11h — Satellite position v3 info (§5.2.11.6)

See [`sat/position_v3.md`](sat/position_v3.md) for the full hand-corrected
table (ephemeris state-vector form; metadata group, ephemeris loop, optional
covariance matrix).

### Table 11i — Interpolation method (§5.2.11.6)

| Value | Method |
|---|---|
| 0 | Reserved |
| 1 | Linear |
| 2 | Lagrange |
| 3 | Reserved |
| 4 | Hermite |
| 5–7 | Reserved |

---
_Hand-transcribed from ETSI EN 300 468 v1.19.1 §5.2.11 (PDF pp. 40-52)._
