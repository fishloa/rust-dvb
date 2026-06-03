# Multiplex Buffer Utilization (tag 0x0C)

**Spec:** ETSI EN 300 468 v1.19.1 ISO/IEC 13818-1
**Parser file:** `crates/dvb_si/src/descriptors/0x0C-multiplex_buffer_utilization.rs`
**Rust struct:** `MultiplexBufferUtilizationDescriptor<'a>`

## Purpose

_TODO: extract purpose from spec for multiplex_buffer_utilization_descriptor_

## Syntax

_TODO: extract from spec_

## Semantics

_TODO: extract semantics from spec_

## Value enumerations

_None defined at this level — see spec._

## Parser requirements

- Verify `descriptor_tag == 0x0C`
- Check `descriptor_length` is within the remaining buffer length
- If `descriptor_length < minimum_size` → return `Error::ShortDescriptor`
- Reserved bits should be ignored during parsing (forward compatibility)

## Byte example

No byte example in spec — TODO: fill from a real capture.

## Cross-references

- Carried by: PMT
- Related descriptors: _see spec §6.1 Table 12_
- Related spec sections: ISO/IEC 13818-1
