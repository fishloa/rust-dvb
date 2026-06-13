//! segmentation_descriptor() — ANSI/SCTE 35 2023r1 §10.3.3, Table 20 (tag 0x02).
//!
//! The richest splice descriptor: carries a segmentation event, its
//! restriction flags, an optional 40-bit `segmentation_duration`, a
//! type/length-prefixed `segmentation_upid()`, the
//! [`SegmentationTypeId`], and an
//! optional `sub_segment_num`/`sub_segments_expected` appendix whose presence
//! is determined by `descriptor_length` (§10.3.3.1).
//!
//! Component Segmentation Mode (`program_segmentation_flag == 0`) is deprecated
//! but parsed/serialized losslessly via [`SegmentationDescriptor::components`].

use super::header::{self, CUEI, HEADER_LEN};
use super::segmentation_enums::{DeviceRestrictions, SegmentationTypeId, SegmentationUpidType};
use crate::error::{Error, Result};
use crate::traits::SpliceDescriptorDef;
use dvb_common::{Parse, Serialize};

/// `splice_descriptor_tag` for segmentation_descriptor (§10.1, Table 16).
pub const TAG: u8 = 0x02;

/// Delivery restriction flags, present when `delivery_not_restricted_flag == 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DeliveryRestrictions {
    /// `web_delivery_allowed_flag`.
    pub web_delivery_allowed: bool,
    /// `no_regional_blackout_flag`.
    pub no_regional_blackout: bool,
    /// `archive_allowed_flag`.
    pub archive_allowed: bool,
    /// `device_restrictions` (Table 21).
    pub device_restrictions: DeviceRestrictions,
}

/// One component entry in the deprecated Component Segmentation Mode loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SegmentationComponent {
    /// 8-bit `component_tag`.
    pub component_tag: u8,
    /// 33-bit `pts_offset` (90 kHz ticks).
    pub pts_offset: u64,
}

/// segmentation_descriptor() — §10.3.3, Table 20.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SegmentationDescriptor<'a> {
    /// 32-bit `identifier` (shall be "CUEI").
    pub identifier: u32,
    /// 32-bit `segmentation_event_id` (§9.9.3).
    pub segmentation_event_id: u32,
    /// When `true`, the named event has been cancelled and no further fields
    /// are present.
    pub segmentation_event_cancel_indicator: bool,
    /// `segmentation_event_id_compliance_indicator`: `false` = compliant.
    pub segmentation_event_id_compliance_indicator: bool,
    /// `program_segmentation_flag`: `true` = Program mode (supported);
    /// `false` = Component mode (deprecated).
    pub program_segmentation_flag: bool,
    /// Delivery restrictions, present when `delivery_not_restricted_flag == 0`;
    /// `None` means delivery is not restricted.
    pub delivery_restrictions: Option<DeliveryRestrictions>,
    /// Component-mode entries, present when `program_segmentation_flag == 0`.
    pub components: Vec<SegmentationComponent>,
    /// 40-bit `segmentation_duration` (90 kHz ticks), present when
    /// `segmentation_duration_flag == 1`.
    pub segmentation_duration: Option<u64>,
    /// `segmentation_upid_type` (Table 22).
    pub segmentation_upid_type: SegmentationUpidType,
    /// `segmentation_upid()` payload bytes (length = `segmentation_upid_length`).
    pub segmentation_upid: &'a [u8],
    /// `segmentation_type_id` (Table 23).
    pub segmentation_type_id: SegmentationTypeId,
    /// `segment_num`.
    pub segment_num: u8,
    /// `segments_expected`.
    pub segments_expected: u8,
    /// `(sub_segment_num, sub_segments_expected)`, present when the
    /// `descriptor_length` includes the optional appendix (§10.3.3.1).
    pub sub_segments: Option<(u8, u8)>,
}

impl<'a> Default for SegmentationDescriptor<'a> {
    fn default() -> Self {
        Self {
            identifier: CUEI,
            segmentation_event_id: 0,
            segmentation_event_cancel_indicator: false,
            segmentation_event_id_compliance_indicator: true,
            program_segmentation_flag: true,
            delivery_restrictions: None,
            components: Vec::new(),
            segmentation_duration: None,
            segmentation_upid_type: SegmentationUpidType::NotUsed,
            segmentation_upid: &[],
            segmentation_type_id: SegmentationTypeId::NotIndicated,
            segment_num: 0,
            segments_expected: 0,
            sub_segments: None,
        }
    }
}

impl<'a> Parse<'a> for SegmentationDescriptor<'a> {
    type Error = Error;
    fn parse(bytes: &'a [u8]) -> Result<Self> {
        let (identifier, body) = header::descriptor_body(bytes, TAG, "segmentation_descriptor")?;
        // segmentation_event_id (4) + flags byte (1).
        if body.len() < 5 {
            return Err(Error::BufferTooShort {
                need: HEADER_LEN + 5,
                have: bytes.len(),
                what: "segmentation_descriptor header",
            });
        }
        let segmentation_event_id = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
        let b = body[4];
        let cancel = b & 0x80 != 0;
        let compliance = b & 0x40 != 0;

        let mut out = Self {
            identifier,
            segmentation_event_id,
            segmentation_event_cancel_indicator: cancel,
            segmentation_event_id_compliance_indicator: compliance,
            ..Self::default()
        };
        if cancel {
            return Ok(out);
        }

        if body.len() < 6 {
            return Err(Error::BufferTooShort {
                need: HEADER_LEN + 6,
                have: bytes.len(),
                what: "segmentation_descriptor flags",
            });
        }
        let flags = body[5];
        out.program_segmentation_flag = flags & 0x80 != 0;
        let duration_flag = flags & 0x40 != 0;
        let delivery_not_restricted = flags & 0x20 != 0;
        if !delivery_not_restricted {
            out.delivery_restrictions = Some(DeliveryRestrictions {
                web_delivery_allowed: flags & 0x10 != 0,
                no_regional_blackout: flags & 0x08 != 0,
                archive_allowed: flags & 0x04 != 0,
                device_restrictions: DeviceRestrictions::from_bits(flags & 0x03),
            });
        }
        let mut pos = 6;

        if !out.program_segmentation_flag {
            if body.len() < pos + 1 {
                return Err(Error::BufferTooShort {
                    need: HEADER_LEN + pos + 1,
                    have: bytes.len(),
                    what: "segmentation_descriptor component_count",
                });
            }
            let count = body[pos] as usize;
            pos += 1;
            for _ in 0..count {
                if body.len() < pos + 6 {
                    return Err(Error::BufferTooShort {
                        need: HEADER_LEN + pos + 6,
                        have: bytes.len(),
                        what: "segmentation_descriptor component",
                    });
                }
                let component_tag = body[pos];
                // 7 reserved bits, then 33-bit pts_offset.
                let pts_offset = ((u64::from(body[pos + 1] & 0x01)) << 32)
                    | (u64::from(body[pos + 2]) << 24)
                    | (u64::from(body[pos + 3]) << 16)
                    | (u64::from(body[pos + 4]) << 8)
                    | u64::from(body[pos + 5]);
                out.components.push(SegmentationComponent {
                    component_tag,
                    pts_offset,
                });
                pos += 6;
            }
        }

        if duration_flag {
            if body.len() < pos + 5 {
                return Err(Error::BufferTooShort {
                    need: HEADER_LEN + pos + 5,
                    have: bytes.len(),
                    what: "segmentation_descriptor segmentation_duration",
                });
            }
            let d = (u64::from(body[pos]) << 32)
                | (u64::from(body[pos + 1]) << 24)
                | (u64::from(body[pos + 2]) << 16)
                | (u64::from(body[pos + 3]) << 8)
                | u64::from(body[pos + 4]);
            out.segmentation_duration = Some(d);
            pos += 5;
        }

        // segmentation_upid_type (1) + segmentation_upid_length (1).
        if body.len() < pos + 2 {
            return Err(Error::BufferTooShort {
                need: HEADER_LEN + pos + 2,
                have: bytes.len(),
                what: "segmentation_descriptor upid header",
            });
        }
        out.segmentation_upid_type = SegmentationUpidType::from_u8(body[pos]);
        let upid_len = body[pos + 1] as usize;
        pos += 2;
        if body.len() < pos + upid_len {
            return Err(Error::LengthOverflow {
                declared: upid_len,
                available: body.len().saturating_sub(pos),
                what: "segmentation_descriptor segmentation_upid",
            });
        }
        out.segmentation_upid = &body[pos..pos + upid_len];
        pos += upid_len;

        // segmentation_type_id (1) + segment_num (1) + segments_expected (1).
        if body.len() < pos + 3 {
            return Err(Error::BufferTooShort {
                need: HEADER_LEN + pos + 3,
                have: bytes.len(),
                what: "segmentation_descriptor type/segment",
            });
        }
        out.segmentation_type_id = SegmentationTypeId::from_u8(body[pos]);
        out.segment_num = body[pos + 1];
        out.segments_expected = body[pos + 2];
        pos += 3;

        // Optional sub_segment appendix — present iff descriptor_length left
        // room for two more bytes (§10.3.3.1).
        if body.len() >= pos + 2 {
            out.sub_segments = Some((body[pos], body[pos + 1]));
            pos += 2;
        }
        // Any trailing bytes within descriptor_length are tolerated but unused.
        let _ = pos;
        Ok(out)
    }
}

impl SegmentationDescriptor<'_> {
    fn body_len(&self) -> usize {
        if self.segmentation_event_cancel_indicator {
            return 5; // event_id (4) + cancel byte (1)
        }
        let mut len = 6; // event_id (4) + cancel byte (1) + flags byte (1)
        if !self.program_segmentation_flag {
            len += 1; // component_count
            len += self.components.len() * 6;
        }
        if self.segmentation_duration.is_some() {
            len += 5;
        }
        len += 2 + self.segmentation_upid.len(); // upid_type + upid_length + upid
        len += 3; // type_id + segment_num + segments_expected
        if self.sub_segments.is_some() {
            len += 2;
        }
        len
    }
}

impl Serialize for SegmentationDescriptor<'_> {
    type Error = Error;
    fn serialized_len(&self) -> usize {
        HEADER_LEN + self.body_len()
    }

    fn serialize_into(&self, buf: &mut [u8]) -> Result<usize> {
        let need = self.serialized_len();
        if buf.len() < need {
            return Err(Error::OutputBufferTooSmall {
                need,
                have: buf.len(),
            });
        }
        let body_len = self.body_len();
        if body_len + 4 > u8::MAX as usize {
            return Err(Error::InvalidValue {
                field: "segmentation_descriptor.descriptor_length",
                reason: "descriptor body exceeds 8-bit descriptor_length",
            });
        }
        header::write_header(buf, TAG, self.identifier, body_len);
        let mut pos = HEADER_LEN;

        buf[pos..pos + 4].copy_from_slice(&self.segmentation_event_id.to_be_bytes());
        // cancel (1) + compliance (1) + 6 reserved bits = 1.
        buf[pos + 4] = (u8::from(self.segmentation_event_cancel_indicator) << 7)
            | (u8::from(self.segmentation_event_id_compliance_indicator) << 6)
            | 0x3F;
        pos += 5;
        if self.segmentation_event_cancel_indicator {
            return Ok(need);
        }

        let duration_flag = self.segmentation_duration.is_some();
        let mut flags =
            (u8::from(self.program_segmentation_flag) << 7) | (u8::from(duration_flag) << 6);
        match &self.delivery_restrictions {
            Some(dr) => {
                // delivery_not_restricted_flag = 0
                flags |= u8::from(dr.web_delivery_allowed) << 4;
                flags |= u8::from(dr.no_regional_blackout) << 3;
                flags |= u8::from(dr.archive_allowed) << 2;
                flags |= dr.device_restrictions.bits() & 0x03;
            }
            None => {
                // delivery_not_restricted_flag = 1, 5 reserved bits = 1.
                flags |= 0x20 | 0x1F;
            }
        }
        buf[pos] = flags;
        pos += 1;

        if !self.program_segmentation_flag {
            buf[pos] = self.components.len() as u8;
            pos += 1;
            for c in &self.components {
                buf[pos] = c.component_tag;
                let o = c.pts_offset & ((1u64 << 33) - 1);
                // 7 reserved bits = 1, then top pts_offset bit.
                buf[pos + 1] = 0xFE | ((o >> 32) as u8 & 0x01);
                buf[pos + 2] = (o >> 24) as u8;
                buf[pos + 3] = (o >> 16) as u8;
                buf[pos + 4] = (o >> 8) as u8;
                buf[pos + 5] = o as u8;
                pos += 6;
            }
        }

        if let Some(d) = self.segmentation_duration {
            let d = d & ((1u64 << 40) - 1);
            buf[pos] = (d >> 32) as u8;
            buf[pos + 1] = (d >> 24) as u8;
            buf[pos + 2] = (d >> 16) as u8;
            buf[pos + 3] = (d >> 8) as u8;
            buf[pos + 4] = d as u8;
            pos += 5;
        }

        buf[pos] = self.segmentation_upid_type.to_u8();
        buf[pos + 1] = self.segmentation_upid.len() as u8;
        pos += 2;
        buf[pos..pos + self.segmentation_upid.len()].copy_from_slice(self.segmentation_upid);
        pos += self.segmentation_upid.len();

        buf[pos] = self.segmentation_type_id.to_u8();
        buf[pos + 1] = self.segment_num;
        buf[pos + 2] = self.segments_expected;
        pos += 3;

        if let Some((sn, se)) = self.sub_segments {
            buf[pos] = sn;
            buf[pos + 1] = se;
            pos += 2;
        }

        debug_assert_eq!(pos, need);
        Ok(need)
    }
}

impl<'a> SpliceDescriptorDef<'a> for SegmentationDescriptor<'a> {
    const TAG: u8 = TAG;
    const NAME: &'static str = "SEGMENTATION";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rt(d: &SegmentationDescriptor) {
        let bytes = d.to_bytes();
        assert_eq!(bytes.len(), d.serialized_len());
        assert_eq!(bytes[0], TAG);
        // descriptor_length counts the bytes after it: identifier (4) + body.
        assert_eq!(bytes[1] as usize, 4 + d.body_len());
        let back = SegmentationDescriptor::parse(&bytes).unwrap();
        assert_eq!(*d, back);
        assert_eq!(back.to_bytes(), bytes);
    }

    #[test]
    fn round_trip_cancel() {
        rt(&SegmentationDescriptor {
            segmentation_event_id: 0x1234,
            segmentation_event_cancel_indicator: true,
            ..Default::default()
        });
    }

    #[test]
    fn round_trip_program_with_duration_and_upid() {
        rt(&SegmentationDescriptor {
            segmentation_event_id: 0x4800_000A,
            segmentation_event_id_compliance_indicator: false,
            program_segmentation_flag: true,
            delivery_restrictions: Some(DeliveryRestrictions {
                web_delivery_allowed: false,
                no_regional_blackout: true,
                archive_allowed: true,
                device_restrictions: DeviceRestrictions::RestrictGroup1,
            }),
            segmentation_duration: Some(90_000 * 30),
            segmentation_upid_type: SegmentationUpidType::AdId,
            segmentation_upid: b"ABCD12345678",
            segmentation_type_id: SegmentationTypeId::ProviderPlacementOpportunityStart,
            segment_num: 1,
            segments_expected: 1,
            sub_segments: Some((1, 2)),
            ..Default::default()
        });
    }

    #[test]
    fn round_trip_no_restrictions_no_subsegments() {
        rt(&SegmentationDescriptor {
            segmentation_event_id: 7,
            delivery_restrictions: None,
            segmentation_type_id: SegmentationTypeId::ProgramStart,
            segment_num: 1,
            segments_expected: 1,
            ..Default::default()
        });
    }

    #[test]
    fn round_trip_component_mode() {
        rt(&SegmentationDescriptor {
            segmentation_event_id: 9,
            program_segmentation_flag: false,
            components: vec![
                SegmentationComponent {
                    component_tag: 1,
                    pts_offset: 0x1_0000,
                },
                SegmentationComponent {
                    component_tag: 2,
                    pts_offset: 0,
                },
            ],
            segmentation_upid_type: SegmentationUpidType::NotUsed,
            segmentation_type_id: SegmentationTypeId::BreakStart,
            ..Default::default()
        });
    }
}
