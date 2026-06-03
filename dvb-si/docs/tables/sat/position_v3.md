# Satellite Position v3 info (table_id 0x4D (subtype 0x04))

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.11.6
**Parser file:** `crates/dvb_si/src/tables/sat/position_v3.rs`
**Rust struct:** `PositionV3`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 11h — Satellite position v3 info
_PDF pages 49-50 (§5.2.11.6)_

| Syntax | Number of Bits | Mnemonic |
|---|---|---|
| usable_stop_time_day_fraction | 32 | spfmsbf |
| } | 16 | uimsbf |
| } | 8 | uimsbf |
| ephemeris_data_count | 7 | bslbf |
| for (j=0; j< ephemeris_data_count; j++) { | 9 | uimsbf |
| epoch_year | 32 | spfmsbf |
| reserved_zero_future_use | 32 | spfmsbf |
| epoch_day | 32 | spfmsbf |
| epoch_day_fraction | 32 | spfmsbf |
| ephemeris_x | 32 | spfmsbf |
| ephemeris_y | 32 | spfmsbf |
| ephemeris_z | 32 | spfmsbf |
| ephemeris_x_dot | 32 | spfmsbf |
| ephemeris_y_dot | 32 | spfmsbf |
| ephemeris_z_dot | 32 | spfmsbf |
| if (ephemeris_accel_flag) { | 8 | uimsbf |
| ephemeris_x_ddot | 7 | bslbf |
| ephemeris_y_ddot | 9 | uimsbf |
| ephemeris_z_ddot | 32 | spfmsbf |
| } | 32 | spfmsbf |
| } |  |  |
| if (covariance_flag == 1) { |  |  |
| covariance_epoch_year |  |  |
| reserved_zero_future_use |  |  |
| covariance_epoch_day |  |  |
| covariance_epoch_day_fraction |  |  |
| for (j=0; j<21; j++) { |  |  |
| covariance_element |  |  |
| } |  |  |
| } |  |  |
| } |  |  |
| } |  |  |

### Table 11i — Interpolation method for ephemeris data
_PDF pages 51-51 (§5.2.11.6)_

| Value | Method |
|---|---|
| 0 | Reserved |
| 1 | Linear |
| 2 | Lagrange |
| 3 | Reserved |
| 4 | Hermite |
| 5 to 7 | Reserved |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.11.6, PDF pages 3-3. 2 tables / 9 rows reproduced verbatim._
