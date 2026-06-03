# Smoothing Buffer (tag 0x10)

**Spec:** ETSI EN 300 468 v1.19.1 ISO/IEC 13818-1
**Parser file:** `crates/dvb_si/src/descriptors/0x10-smoothing_buffer.rs`
**Rust struct:** `SmoothingBufferDescriptor<'a>`

## Purpose

6.2.38        Short smoothing buffer descriptor ............................................................................................................ 103 6.2.39        Stream identifier descriptor....................................................................................................................... 105 6.2.40        Stuffing descriptor .................................................................................................................................... 105

## Syntax

_TODO: syntax table could not be auto-extracted — verify against spec_


## Semantics

_TODO: extract semantics from spec_

## Value enumerations

_No value enumerations defined at this level — see spec._

## Parser requirements

- Verify `descriptor_tag == 0x10`
- Check `descriptor_length` is within the remaining buffer length
- If `descriptor_length < minimum_size` → return `Error::ShortDescriptor`
- Reserved bits should be ignored during parsing (forward compatibility)

## Byte example

No byte example in spec — TODO: fill from a real capture.

## Cross-references

- Carried by: PMT
- Related descriptors: _see spec §6.1 Table 12_
- Related spec sections: ISO/IEC 13818-1
