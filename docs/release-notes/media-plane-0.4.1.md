# media-plane 0.4.1

**Release date:** 2026-08-16

Additive patch — one new public getter, no breaking changes.

## What's new

- `Trunk::time_anchor()` — returns the `TimeAnchor` the trunk's event log
  has been given via `SegmentWriter::set_time_anchor`, or `None` until one
  has been set. Added for `hls-runtime` 0.7.0's SCTE-35 → `EXT-X-DATERANGE`
  rendering (issue #965), which needs the wall-clock mapping to build a
  `timed_metadata::Timeline`.

## Migration

No breaking changes; drop-in replacement for 0.4.0.
