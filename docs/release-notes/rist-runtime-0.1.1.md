# rist-runtime 0.1.1 — 2026-08-30

Patch release: two DoS-class fixes in the ARQ engine.

## Fixed

- **NACK amplification** (#977). `arq::Sender::on_range_nack` /
  `on_generic_nack` did an O(n) linear scan per expanded sequence number.
  An adversarial RangeNack (`Additional = 0xFFFF`) against a deep buffer
  multiplied into tens of millions of comparisons. The lookup buffer is now
  indexed by sequence number (`BTreeMap`, stays `no_std`+`alloc`), turning
  each lookup into O(log n).

- **Receiver sequence-jump flooding** (#978). `arq::Receiver::feed`
  backfilled one `MissingState` BTreeMap entry per skipped sequence number
  with no cap. A single spoofed packet claiming a sequence number tens of
  thousands ahead inserted tens of thousands of entries. Gaps wider than
  `MAX_GAP` (512) are now treated as a stream reset instead of being
  backfilled.

## Compatibility

No breaking changes. MSRV 1.95.0.
