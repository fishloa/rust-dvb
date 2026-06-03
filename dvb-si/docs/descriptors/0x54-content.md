# Content Descriptor (tag 0x54)

**Spec:** ETSI EN 300 468 v1.19.1 §6.2.9
**Parser file:** `crates/dvb_si/src/descriptors/content.rs`
**Rust struct:** `ContentDescriptor<'a>`

## Tables

### Table 27 — NGA component_type value assignments
_PDF pages 67-67 (§6.2.9)_

| stream_content | stream_content_ext | component_type | Description |
|---|---|---|---|
| 0xC to 0xF | n/a | 0x00 to 0xFF | user defined |
| NOTE 1: In order to maintain backwards compatibility, the value of the stream_content_ext field is not applicable (n/a) |  |  |  |
| for stream_content values in the range 0x1 to 0x8, and is set to 0xF. |  |  |  |
| NOTE 2: The terms "standard definition", "high definition", "ultra high definition", "25 Hz", "30 Hz", "50 Hz", and |  |  |  |
| "60 Hz" are used as defined in ETSI TS 101 154 [14], clauses 5.1 to 5.4 for MPEG-2 and clauses 5.5 to 5.7 |  |  |  |
| for H.264/AVC, and clauses 5.14.2 and 5.14.3 for HEVC respectively. The terms "HDR", "HLG10" and |  |  |  |
| "PQ10" are used as defined in clause 5.14.3 of ETSI TS 101 154 [14]. |  |  |  |
| NOTE 3: The specific audio description types indicating the use of broadcast-mix or receiver-mix audio should be |  |  |  |
| preferred over these generic types. For more details see annex J. |  |  |  |
| NOTE 4: These component_descriptor values are intended to be present in combination with one or more |  |  |  |
| component descriptors with the same component_tag value. See annex N for example uses. |  |  |  |
| NOTE 5: See ETSI TS 101 547-3 [17] for further information on stereoscopic modes. |  |  |  |
| NOTE 6: 24 Hz video will also use this component_type. |  |  |  |
| NOTE 7: Audio streams using AAC audio shall use the corresponding HE AAC values. The AAC profile includes low- |  |  |  |
| complexity AAC. |  |  |  |
| NOTE 8: The specific audio description types indicating the use of broadcast-mix or receiver-mix audio should be |  |  |  |
| preferred over these generic types. For more details see annex J. |  |  |  |
| NOTE 9: For rules on the use of these values, see clause I.2.5 and ETSI TS 101 547-4 [18]. |  |  |  |
| NOTE 10: This value should be used for backward compatible Hybrid Log Gamma with a bit depth of 10 bits (HLG10) |  |  |  |
| HDR services, and/or backward compatible High Frame Rate (HFR) services which are decodable by |  |  |  |
| HEVC_UHDTV_IRD as defined in ETSI TS 101 154 [14]. See also clause I.2.5.2. |  |  |  |
| NOTE 11: For the rules on the use of these values, see clause I.2.6. |  |  |  |
| NOTE 12: These values should be used for elementary streams that convey a single presentation only. |  |  |  |
| NOTE 13: For the rules on the use of these values, see clause I.2.5.2. |  |  |  |
| NOTE 14: For the rules on the use of these values, see clause I.2.8. |  |  |  |
| NOTE 15: For the rules on the use of these values, see clause I.2.9. |  |  |  |
| component_type bits |  | Description |  |
| b(cid:8) (msb) |  | reserved_zero_future_use |  |
| b(cid:7) |  | content is pre-rendered for consumption with headphones |  |
| b(cid:6) |  | content enables interactivity |  |
| b(cid:5) |  | content enables dialogue enhancement (see note) |  |
| b(cid:4) |  | content contains spoken subtitles |  |
| b(cid:3) |  | content contains audio description |  |
| b(cid:2) | b(cid:0) | preferred reproduction channel layout |  |
| 0b0 | 0b0 | no preference |  |
| 0b0 | 0b1 | stereo |  |
| 0b1 | 0b0 | two-dimensional |  |
| 0b1 | 0b1 | three-dimensional |  |
| NOTE: Content enabling dialogue enhancement also offers support for clean audio for the hearing impaired. |  |  |  |

### Table 28 — Content descriptor
_PDF pages 68-68 (§6.2.9)_

| Syntax | Number of bits | Identifier |
|---|---|---|
| content_descriptor() { | 8 | uimsbf |
| descriptor_tag | 8 | uimsbf |
| descriptor_length | 4 | uimsbf |
| for (i=0;i<N;i++) { | 4 | uimsbf |
| content_nibble_level_1 | 8 | uimsbf |
| content_nibble_level_2 |  |  |
| user_byte |  |  |
| } |  |  |
| } |  |  |

### Table 29 — Content nibble level 1 and 2 assignments
_PDF pages 68-69 (§6.2.9)_

| content_nibble_level_1 | content_nibble_level_2 | Description |  |
|---|---|---|---|
| 0x0 | 0x0 to 0xF | undefined content |  |
| Movie/Drama: |  |  |  |
| 0x1 | 0x0 | movie/drama (general) |  |
| 0x1 | 0x1 | detective/thriller |  |
| 0x1 | 0x2 | adventure/western/war |  |
| 0x1 | 0x3 | science fiction/fantasy/horror |  |
| 0x1 | 0x4 | comedy |  |
| 0x1 | 0x5 | soap/melodrama/folkloric |  |
| 0x1 | 0x6 | romance |  |
| 0x1 | 0x7 | serious/classical/religious/historical movie/drama |  |
| 0x1 | 0x8 | adult movie/drama |  |
| 0x1 | 0x9 to 0xE | reserved for future use |  |
| 0x1 | 0xF | user defined |  |
| News/Current affairs: |  |  |  |
| 0x2 | 0x0 | news/current affairs (general) |  |
| 0x2 | 0x1 | news/weather report |  |
| 0x2 | 0x2 | news magazine |  |
| 0x2 | 0x3 | documentary |  |
| 0x2 | 0x4 | discussion/interview/debate |  |
| 0x2 | 0x5 to 0xE | reserved for future use |  |
| 0x2 | 0xF | user defined |  |
| Show/Game show: |  |  |  |
| 0x3 | 0x0 | show/game show (general) |  |
| 0x3 | 0x1 | game show/quiz/contest |  |
| 0x3 | 0x2 | variety show |  |
| 0x3 | 0x3 | talk show |  |
| 0x3 | 0x4 to 0xE | reserved for future use |  |
| 0x3 | 0xF | user defined |  |
| Sports: |  |  |  |
| 0x4 | 0x0 | sports (general) |  |
| 0x4 | 0x1 | special events (Olympic Games, World Cup, etc.) |  |
| 0x4 | 0x2 | sports magazines |  |
| 0x4 | 0x3 | football/soccer |  |
| 0x4 | 0x4 | tennis/squash |  |
| 0x4 | 0x5 | team sports (excluding football) |  |
| 0x4 | 0x6 | athletics |  |
| 0x4 | 0x7 | motor sport |  |
| 0x4 | 0x8 | water sport |  |
| 0x4 | 0x9 | winter sports |  |
| 0x4 | 0xA | equestrian |  |
| 0x4 | 0xB | martial sports |  |
| 0x4 | 0xC to 0xE | reserved for future use |  |
| 0x4 | 0xF | user defined |  |
| Children's/Youth programmes: |  |  |  |
| 0x5 | 0x0 | children's/youth programmes (general) |  |
| 0x5 | 0x1 | pre-school children's programmes |  |
| 0x5 | 0x2 | entertainment programmes for 6 to 14 |  |
| 0x5 | 0x3 | entertainment programmes for 10 to 16 |  |
| 0x5 | 0x4 | informational/educational/school programmes |  |
| 0x5 | 0x5 | cartoons/puppets |  |
| 0x5 | 0x6 to 0xE | reserved for future use |  |
| 0x5 | 0xF | user defined |  |
| Music/Ballet/Dance: |  |  |  |
| 0x6 | 0x0 | music/ballet/dance (general) |  |
| 0x6 | 0x1 | rock/pop |  |
| 0x6 | 0x2 | serious music/classical music |  |
| 0x6 | 0x3 | folk/traditional music |  |
| 0x6 | 0x4 | jazz |  |
| 0x6 | 0x5 | musical/opera |  |
| 0x6 | 0x6 | ballet |  |
| 0x6 | 0x7 to 0xE | reserved for future use |  |
| 0x6 | 0xF | user defined |  |
| Arts/Culture (without music): |  |  |  |
| 0x7 | 0x0 | arts/culture (without music, general) |  |
| 0x7 | 0x1 | performing arts |  |
| 0x7 | 0x2 | fine arts |  |
| 0x7 | 0x3 | religion |  |
| 0x7 | 0x4 | popular culture/traditional arts |  |
| 0x7 | 0x5 | literature |  |
| 0x7 | 0x6 | film/cinema |  |
| 0x7 | 0x7 | experimental film/video |  |
| 0x7 | 0x8 | broadcasting/press |  |
| 0x7 | 0x9 | new media |  |
| 0x7 | 0xA | arts/culture magazines |  |
| 0x7 | 0xB | fashion |  |
| 0x7 | 0xC to 0xE | reserved for future use |  |
| 0x7 | 0xF | user defined |  |
| Social/Political issues/Economics: |  |  |  |
| 0x8 | 0x0 | social/political issues/economics (general) |  |
| 0x8 | 0x1 | magazines/reports/documentary |  |
| 0x8 | 0x2 | economics/social advisory |  |
| 0x8 | 0x3 | remarkable people |  |
| 0x8 | 0x4 to 0xE | reserved for future use |  |
| 0x8 | 0xF | user defined |  |
| Education/Science/Factual topics: |  |  |  |
| 0x9 | 0x0 | education/science/factual topics (general) |  |
| 0x9 | 0x1 | nature/animals/environment |  |
| 0x9 | 0x2 | technology/natural sciences |  |
| 0x9 | 0x3 | medicine/physiology/psychology |  |
| 0x9 | 0x4 | foreign countries/expeditions |  |
| 0x9 | 0x5 | social/spiritual sciences |  |
| 0x9 | 0x6 | further education |  |
| 0x9 | 0x7 | languages |  |
| 0x9 | 0x8 to 0xE | reserved for future use |  |
| 0x9 | 0xF | user defined |  |
| Leisure hobbies: |  |  |  |
| 0xA | 0x0 | leisure hobbies (general) |  |
| 0xA | 0x1 | tourism/travel |  |
| 0xA | 0x2 | handicraft |  |
| 0xA | 0x3 | motoring |  |
| 0xA | 0x4 | fitness and health |  |

---
_Rendered from ETSI EN 300 468 v1.19.1 §6.2.9, PDF pages 4-4. 3 tables / 120 rows reproduced verbatim._
