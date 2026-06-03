# Time Association info (table_id 0x4D (subtype 0x02))

**Spec:** ETSI EN 300 468 v1.19.1 §5.2.11.4
**Parser file:** `crates/dvb_si/src/tables/sat/time_association.rs`
**Rust struct:** `TimeAssociation`

## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 11e — Time association info
_PDF pages 45-45 (§5.2.11.4)_

| Syntax | Number | Mnemonic |
|---|---|---|
|  | of bits |  |
| time_association_info(){ |  |  |
| association_type | 4 | uimsbf |
| if (association_type = 1) { |  |  |
| leap59 | 1 | bsblf |
| leap61 | 1 | bsblf |
| pastleap59 | 1 | bsblf |
| pastleap61 | 1 | bsblf |
| } else { |  |  |
| reserved_zero_future_use | 4 | bsblf |
| } |  |  |
| ncr_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| ncr_ext | 9 | uimsbf |
| association_timestamp_seconds | 64 | uimsbf |
| association_timestamp_nanoseconds | 32 | uimsbf |
| } |  |  |

### Table 11f — Association type coding
_PDF pages 45-45 (§5.2.11.4)_

| association_type | Description |
|---|---|
| 0 | UTC without leap second signalling |
| 1 | UTC with leap second signalling |
| 2 to 15 | reserved |

---
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.11.4, PDF pages 3-3. 2 tables / 21 rows reproduced verbatim._
