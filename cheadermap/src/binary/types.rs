// Copyright (c) 2022 Milen Dzhumerov

pub type MagicType = u32;
pub type VersionType = u16;
pub type ReservedType = u16;
pub type StringSectionOffsetType = u32;
pub type StringCountType = u32;
pub type BucketCountType = u32;
pub type MaxValueLength = u32;

pub const STRING_SECTION_OFFSET_RESERVED: StringSectionOffsetType = 0;
pub const VERSION_1: VersionType = 1;
pub const RESERVED: ReservedType = 0;
