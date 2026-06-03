# SAT — Satellite Access Tables (§5.2.11)


## Spec text

Contents
Intellectual Property Rights ................................................................................................................................ 8
Foreword ............................................................................................................................................................. 8
Modal verbs terminology .................................................................................................................................... 9
1 Scope ...................................................................................................................................................... 10
2 References .............................................................................................................................................. 10

## Tables

### Table 11a — Satellite access section
_PDF pages 40-40 (§5.2.11.1)_

| Syntax | Number | Identifier |
|---|---|---|
|  | of bits |  |
| satellite_access_section() { | 8 | uimsbf |
| table_id | 1 | bslbf |
| section_syntax_indicator | 1 | bslbf |
| private_indicator | 2 | bslbf |
| reserved | 12 | uimsbf |
| section_length | 6 | uimsbf |
| satellite_table_id | 10 | uimsbf |
| table_count | 2 | bslbf |
| reserved | 5 | uimsbf |
| version_number | 1 | bslbf |
| current_next_indicator | 8 | uimsbf |
| section_number | 8 | uimsbf |
| last_section_number | 8 | bslbf |
| reserved_zero_future_use | 8 | bslbf |
| if (satellite_table_id == 0) { | 32 | rpchof |
| satellite_position_v2_info() |  |  |
| } |  |  |
| else if (satellite_table_id == 1) { |  |  |
| cell_fragment_info() |  |  |
| } |  |  |
| else if (satellite_table_id == 2) { |  |  |
| time_association_info() |  |  |
| } |  |  |
| else if (satellite_table_id == 3) { |  |  |
| beamhopping_time_plan_info() |  |  |
| } |  |  |
| else if (satellite_table_id == 4) { |  |  |
| satellite_position_v3_info() |  |  |
| } |  |  |
| else { |  |  |
| for (i=0;i<N;i++) { |  |  |
| reserved_zero_future_use |  |  |
| } |  |  |
| } |  |  |
| CRC_32 |  |  |
| } |  |  |

### Table 11b — Satellite table id coding
_PDF pages 40-40 (§5.2.11.1)_

| satellite_table_id | Syntax | Defined in |
|---|---|---|
| 0 | satellite_position_v2_info | clause 5.2.11.2 |
| 1 | cell_fragment_info | clause 5.2.11.3 |
| 2 | time_association_info | clause 5.2.11.4 |
| 3 | beamhopping_time_plan_info | clause 5.2.11.5 |
| 4 | satellite_position_v3_info | clause 5.2.11.5 |
| 5 to 63 | reserved for future use |  |

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

### Table 11d — Cell fragment info
_PDF pages 43-43 (§5.2.11.3)_

| Syntax | Number | Mnemonic |
|---|---|---|
|  | of bits |  |
| cell_fragment_info(){ |  |  |
| for (i=1;i<=N;i++) { |  |  |
| cell_fragment_id | 32 | uimsbf |
| first_occurence | 1 | bsblf |
| last_occurence | 1 | bsblf |
| if (first_occurence == 1) { |  |  |
| reserved_zero_future_use | 4 | bsblf |
| center_latitude | 18 | tcimsbf |
| reserved_zero_future_use | 5 | bsblf |
| center_longitude | 19 | tcimsbf |
| max_distance | 24 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| } else { |  |  |
| reserved_zero_future_use | 4 | bsblf |
| } |  |  |
| delivery_system_id_loop_count | 10 | uimsbf |
| for (j=0;j<delivery_system_id_loop_count;j++) { |  |  |
| delivery_system_id | 32 | uimsbf |
| } |  |  |
| reserved_zero_future_use | 6 | bsblf |
| new_delivery_system_id_loop_count | 10 | uimsbf |
| for (k=0;k<new_delivery_system_id_loop_count;k++) { |  |  |
| new_delivery_system_id | 32 | uimsbf |
| time_of_application_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| time_of_application_ext | 9 | uimsbf |
| } |  |  |
| reserved_zero_future_use | 6 | bsblf |
| obsolescent_delivery_system_id_loop_count | 10 | uimsbf |
| for (l=0;l<obsolescent_delivery_system_id_loop_count;l++) { |  |  |
| obsolescent_delivery_system_id | 32 | uimsbf |
| time_of_obsolescence_base | 33 | uimsbf |
| reserved_zero_future_use | 6 | bsblf |
| time_of_obsolescence_ext | 9 | uimsbf |
| } |  |  |
| } |  |  |
| } |  |  |

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
_Rendered from ETSI EN 300 468 v1.19.1 §5.2.11, PDF pages 3-3. 9 tables / 154 rows reproduced verbatim._
