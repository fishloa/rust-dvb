# transmux 0.24.1 — 2026-08-30

Patch release: 16 audit findings fixed. No breaking changes, no new API
surface. All findings from the Fable 5 static analysis audit (#999).

## Fixed

- **OOM guards** for unbounded wire-driven allocations (#983, #987, #988):
  `progressive_demux` `total_samples` (stsc × stco/co64 sum) bounded against
  file size; `sample_groups` sgpd/sbgp/subs `entry_count` and `movie_fragment`
  trun `sample_count` guarded by `bounded_entry_count`.

- **Parse panic on short CENC input** (#989): `SchemeInformationBox::parse`
  and `ProtectionSchemeInfoBox::parse` panicked on input shorter than
  `BOX_HDR`. Now returns `BufferTooShort`.

- **Silent truncation/overflow** (#981, #984, #996, #997, #998): ADTS 13-bit
  frame_len validation; AMF0 string u16::MAX length check; composition_offset
  clamped to i32 range; AVC dimensions capped to u16::MAX; WebM cluster
  timestamp i64 overflow guarded.

- **MKV u64 offsets** (#995): cluster offsets widened from u32 to u64,
  preventing wrap on files > 4 GB.

- **Reserved field zeroing** (#986): mvhd/tkhd `serialize_into` now
  zero-fills all reserved byte regions.

- **CENC seig detection** (#990): content using `seig` sample-group key
  rotation is now detected and explicitly rejected with
  `Error::UnsupportedFeature` instead of silently decrypting with the wrong
  key.

- **mdat lower-bound validation** (#991): `data_offset` values pointing into
  the moof itself are now rejected.

- **Splice match_tracks** (#992): track matching prevents 2-to-1 mapping in
  dual-audio content.

- **Repackage presentation_times** (#993): uses absolute dts when available.

- **LL-DASH Timeline addressing** (#994): `inject_ll` matches non-self-closing
  `<SegmentTemplate>` form.

## Compatibility

No breaking changes — bugfix only. MSRV 1.95.0, edition 2024.
