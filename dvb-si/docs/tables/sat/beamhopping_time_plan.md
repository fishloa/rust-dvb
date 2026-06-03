# Beamhopping Time Plan info (table_id 0x4D (subtype 0x03))

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.11.5
**Parser file:** `crates/dvb_si/src/tables/sat/beamhopping_time_plan.rs`
**Rust struct:** `BeamhoppingTimePlan`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 11g — Beamhopping time plan info
_PDF pages 47-47 (§5.2.11.5)_

| Syntax | Number | Mnemonic |
|---|---|---|
|  | of bits |  |
| beamhopping_time_plan_info() { |  |  |
| for (i=1;i<=N;i++) { |  |  |
| beamhopping_time_plan_id | 32 | uimsbf |
| reserved_zero_future_use | 4 | bsblf |
| beamhopping_time_plan_length | 12 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| time_plan_mode | 2 | uimsbf |
| time_of_application_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| time_of_application_ext | 9 | uimsbf |
| cycle_duration_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| cycle_duration_ext | 9 | uimsbf |
| if time_plan_mode == 0 { |  |  |
| dwell_duration_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| dwell_duration_ext | 9 | uimsbf |
| on_time_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| on_time_ext | 9 | uimsbf |
| } |  |  |
| if (time_plan_mode == 1) { |  |  |
| reserved_zero_future_use | 1 | bsblf |
| bit_map_size | 15 | uimsbf |
| reserved_zero_future_use | 1 | bsblf |
| current_slot | 15 | uimsbf |
| for (j=1;j<=bit_map_size;j++) { |  |  |
| slot_transmission_on | 1 | bsblf |
| } |  |  |
| for (k=1;k<=J;k++) { |  |  |
| padding_bit | 1 | bsblf |
| } |  |  |
| } |  |  |
| if (time_plan_mode == 2) { |  |  |
| grid_size_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| grid_size_ext | 9 | uimsbf |
| revisit_duration_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| revisit_duration_ext | 9 | uimsbf |
| sleep_time_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | uimsbf |
| sleep_time_ext | 9 | bsblf |
| sleep_duration_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| sleep_duration_ext | 9 | uimsbf |
| } |  |  |
| } |  |  |
| } |  |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.11.5, PDF pages 3-3. 1 tables / 50 rows reproduced verbatim._
