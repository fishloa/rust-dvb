# Satellite Position v2 info (table_id 0x4D (subtype 0x00))

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.11.2
**Parser file:** `crates/dvb_si/src/tables/sat/position_v2.rs`
**Rust struct:** `PositionV2`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 11c — Satellite position v2 info
_PDF pages 42-42 (§5.2.11.2)_

| Syntax | Number of | Mnemonic |
|---|---|---|
|  | bits |  |
| satellite_position_v2_info() { |  |  |
| for (i=1;i<=N;i++) { |  |  |
| satellite_id | 24 |  |
| reserved_zero_future_use | 7 |  |
| position_system | 1 |  |
| if (position_system == 0) { |  |  |
| orbital position | 16 | bslbf |
| west_east_flag | 1 | bslbf |
| reserved_zero_future_use | 7 | bslbf |
| } |  |  |
| if (position_system == 1) { |  |  |
| epoch_year | 8 | uimsbf |
| day_of_the_year | 16 | uimsbf |
| day_fraction | 32 | spfmsbf |
| mean_motion_first_derivative | 32 | spfmsbf |
| mean_motion_second_derivative | 32 | spfmsbf |
| drag_term | 32 | spfmsbf |
| inclination | 32 | spfmsbf |
| right_ascension_of_the_ascending_node | 32 | spfmsbf |
| eccentricity | 32 | spfmsbf |
| argument_of_perigree | 32 | spfmsbf |
| mean_anomaly | 32 | spfmsbf |
| mean_motion | 32 | spfmsbf |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.11.2, PDF pages 3-3. 1 tables / 27 rows reproduced verbatim._
