
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stream_2/Stream_PositionScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Stream_PositionScope {
    pub m_stream: crate::app::stream_2::Stream_2,
    pub m_position: i32,
}

impl ::unity2::ClassIdentity for Stream_PositionScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Stream.PositionScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Stream_PositionScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-stream_2")]
#[::unity2::methods(value)]
impl Stream_PositionScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2, position: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stream_2/Stream_2.md")))]
#[::unity2::class(namespace = "App", name = "Stream")]
#[parent(crate::system::object::Object)]
pub struct Stream_2 {
    #[rename(name = "m_Stack")]
    pub m_stack:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::stream_2::Stream_Info>,
    #[static_field]
    #[rename(name = "MAGIC_CODE")]
    pub magic_code: u32,
    #[static_field]
    #[rename(name = "Magic_Number_Compress")]
    pub magic_number_compress: u64,
    #[static_field]
    #[rename(name = "HashCode")]
    pub hash_code: u16,
    #[static_field]
    #[rename(name = "NullCode")]
    pub null_code: u16,
}

#[cfg(feature = "app-stream_2")]
#[::unity2::methods]
impl Stream_2 {
    #[method(name = "get_m_Buffer", args = 0)]
    pub fn get_m_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "set_m_Buffer", args = 1)]
    pub fn set_m_buffer(self, value: ::unity2::Array<u8>) -> ();

    #[method(name = "get_m_Position", args = 0)]
    pub fn get_m_position(self) -> i32;

    #[method(name = "set_m_Position", args = 1)]
    pub fn set_m_position(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, size: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, buffer: ::unity2::Array<u8>) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, size: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> i32;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: i32) -> ();

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "HasNext", args = 0)]
    pub fn has_next(self) -> bool;

    #[method(name = "HasStream", args = 0)]
    pub fn has_stream(self) -> bool;

    #[method(name = "GetRest", args = 0)]
    pub fn get_rest(self) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Fill", args = 2)]
    pub fn fill(self, data: u8, size: i32) -> ();

    #[method(name = "WriteBool", args = 1)]
    pub fn write_bool(self, data: bool) -> ();

    #[method(name = "WriteByte", args = 1)]
    pub fn write_byte(self, data: u8) -> ();

    #[method(name = "WriteSbyte", args = 1)]
    pub fn write_sbyte(self, data: i8) -> ();

    #[method(name = "Write8", args = 1)]
    pub fn write8(self, data: u8) -> ();

    #[method(name = "Write8", args = 1)]
    pub fn write8_2(self, data: i8) -> ();

    #[method(name = "WriteShort", args = 1)]
    pub fn write_short(self, data: i16) -> ();

    #[method(name = "WriteUshort", args = 1)]
    pub fn write_ushort(self, data: u16) -> ();

    #[method(name = "Write16", args = 1)]
    pub fn write16(self, data: i16) -> ();

    #[method(name = "Write16", args = 1)]
    pub fn write16_2(self, data: u16) -> ();

    #[method(name = "WriteInt", args = 1)]
    pub fn write_int(self, data: i32) -> ();

    #[method(name = "WriteUint", args = 1)]
    pub fn write_uint(self, data: u32) -> ();

    #[method(name = "Write32", args = 1)]
    pub fn write32(self, data: i32) -> ();

    #[method(name = "Write32", args = 1)]
    pub fn write32_2(self, data: u32) -> ();

    #[method(name = "WriteHash", args = 1)]
    pub fn write_hash(self, data: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteLong", args = 1)]
    pub fn write_long(self, data: i64) -> ();

    #[method(name = "WriteUlong", args = 1)]
    pub fn write_ulong(self, data: u64) -> ();

    #[method(name = "Write64", args = 1)]
    pub fn write64(self, data: i64) -> ();

    #[method(name = "Write64", args = 1)]
    pub fn write64_2(self, data: u64) -> ();

    #[method(name = "WriteFloat", args = 1)]
    pub fn write_float(self, data: f32) -> ();

    #[method(name = "WriteF32", args = 1)]
    pub fn write_f32(self, data: f32) -> ();

    #[method(name = "WriteVector2", args = 1)]
    pub fn write_vector2(self, data: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "WriteVector3", args = 1)]
    pub fn write_vector3(self, data: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "WriteQuaternion", args = 1)]
    pub fn write_quaternion(self, data: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "WriteColor", args = 1)]
    pub fn write_color(self, data: crate::unity_engine::color::Color) -> ();

    #[method(name = "WriteBlock", args = 1)]
    pub fn write_block(self, data: ::unity2::Array<u8>) -> ();

    #[method(name = "WriteBlock", args = 2)]
    pub fn write_block_2(self, data: ::unity2::Array<u8>, size: i32) -> ();

    #[method(name = "WriteStream", args = 1)]
    pub fn write_stream(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteString", args = 1)]
    pub fn write_string(self, data: ::unity2::Il2CppString) -> ();

    #[method(name = "Skip", args = 1)]
    pub fn skip(self, size: i32) -> ();

    #[method(name = "Replace", args = 1)]
    pub fn replace(self, position: i32) -> i32;

    #[method(name = "PeekByte", args = 0)]
    pub fn peek_byte(self) -> u8;

    #[method(name = "Peek8", args = 0)]
    pub fn peek8(self) -> u8;

    #[method(name = "ReadBool", args = 0)]
    pub fn read_bool(self) -> bool;

    #[method(name = "ReadByte", args = 0)]
    pub fn read_byte(self) -> u8;

    #[method(name = "ReadSbyte", args = 0)]
    pub fn read_sbyte(self) -> i8;

    #[method(name = "Read8", args = 0)]
    pub fn read8(self) -> u8;

    #[method(name = "Read8", args = 1)]
    pub fn read8_2(self, position: i32) -> u8;

    #[method(name = "ReadShort", args = 0)]
    pub fn read_short(self) -> i16;

    #[method(name = "ReadUshort", args = 0)]
    pub fn read_ushort(self) -> u16;

    #[method(name = "Read16", args = 0)]
    pub fn read16(self) -> i16;

    #[method(name = "Read16", args = 1)]
    pub fn read16_2(self, position: i32) -> i16;

    #[method(name = "ReadInt", args = 0)]
    pub fn read_int(self) -> i32;

    #[method(name = "ReadUint", args = 0)]
    pub fn read_uint(self) -> u32;

    #[method(name = "ReadHash", args = 0)]
    pub fn read_hash(self) -> i32;

    #[method(name = "Read32", args = 0)]
    pub fn read32(self) -> i32;

    #[method(name = "Read32", args = 1)]
    pub fn read32_2(self, position: i32) -> i32;

    #[method(name = "ReadLong", args = 0)]
    pub fn read_long(self) -> i64;

    #[method(name = "Read64", args = 0)]
    pub fn read64(self) -> i64;

    #[method(name = "ReadUlong", args = 0)]
    pub fn read_ulong(self) -> u64;

    #[method(name = "ReadUlong", args = 1)]
    pub fn read_ulong_2(self, position: i32) -> u64;

    #[method(name = "ReadFloat", args = 0)]
    pub fn read_float(self) -> f32;

    #[method(name = "ReadF32", args = 0)]
    pub fn read_f32(self) -> f32;

    #[method(name = "ReadVector2", args = 0)]
    pub fn read_vector2(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "ReadVector3", args = 0)]
    pub fn read_vector3(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ReadQuaternion", args = 0)]
    pub fn read_quaternion(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "ReadColor", args = 0)]
    pub fn read_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "ReadBlock", args = 0)]
    pub fn read_block(self) -> ::unity2::Array<u8>;

    #[method(name = "ReadBlock", args = 1)]
    pub fn read_block_2(self, bytes: ::unity2::Array<u8>) -> i32;

    #[method(name = "ReadStream", args = 1)]
    pub fn read_stream(self, stream: crate::app::stream_2::Stream_2) -> i32;

    #[method(name = "ReadString", args = 0)]
    pub fn read_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadString", args = 1)]
    pub fn read_string_2(self, empty: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Write8", args = 2)]
    pub fn write8_3(self, data: u8, position: i32) -> ();

    #[method(name = "Write16", args = 2)]
    pub fn write16_3(self, data: i16, position: i32) -> ();

    #[method(name = "Write32", args = 2)]
    pub fn write32_3(self, data: i32, position: i32) -> ();

    #[method(name = "WriteInt", args = 2)]
    pub fn write_int_2(self, data: i32, position: i32) -> ();

    #[method(name = "WriteBegin", args = 1)]
    pub fn write_begin(self, version: i32) -> ();

    #[method(name = "WriteEnd", args = 0)]
    pub fn write_end(self) -> ();

    #[method(name = "ReadBegin", args = 1)]
    pub fn read_begin(self, version: i32) -> bool;

    #[method(name = "ReadBegin", args = 0)]
    pub fn read_begin_2(self) -> i32;

    #[method(name = "ReadEnd", args = 1)]
    pub fn read_end(self, is_warning: bool) -> ();

    #[method(name = "ReadSkip", args = 0)]
    pub fn read_skip(self) -> ();

    #[method(name = "WriteCrc32", args = 0)]
    pub fn write_crc32(self) -> ();

    #[method(name = "ReadCrc32", args = 0)]
    pub fn read_crc32(self) -> bool;

    #[method(name = "CalcCrc32", args = 0)]
    pub fn calc_crc32(self) -> u32;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Compress", args = 1)]
    pub fn compress(self, offset: i32) -> bool;

    #[method(name = "Decompress", args = 1)]
    pub fn decompress(self, offset: i32) -> bool;

    #[method(name = "TestCompress", args = 0)]
    pub fn test_compress(self) -> ();

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-stream_2")]
impl Stream_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stream_2),
                ::core::stringify!(new),
            )
        });
        <Self as IStream_2Methods>::ctor(this);
        this
    }

    pub fn new_2(size: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stream_2),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStream_2Methods>::ctor_2(this, size);
        this
    }

    pub fn new_3(buffer: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stream_2),
                ::core::stringify!(new_3),
            )
        });
        <Self as IStream_2Methods>::ctor_3(this, buffer);
        this
    }

    pub fn new_4(stream: crate::app::stream_2::Stream_2) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stream_2),
                ::core::stringify!(new_4),
            )
        });
        <Self as IStream_2Methods>::ctor_4(this, stream);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stream_2/Stream_Info.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Stream_Info {
    pub position: i32,
    pub version: i32,
    pub size: i32,
}

impl ::unity2::ClassIdentity for Stream_Info {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Stream.Info";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Stream_Info {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stream_2/Stream_ReadScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Stream_ReadScope {
    pub m_stream: crate::app::stream_2::Stream_2,
}

impl ::unity2::ClassIdentity for Stream_ReadScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Stream.ReadScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Stream_ReadScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-stream_2")]
#[::unity2::methods(value)]
impl Stream_ReadScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "set_Version", args = 1)]
    pub fn set_version(self, value: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stream_2/Stream_WriteScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Stream_WriteScope {
    pub m_stream: crate::app::stream_2::Stream_2,
}

impl ::unity2::ClassIdentity for Stream_WriteScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Stream.WriteScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Stream_WriteScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-stream_2")]
#[::unity2::methods(value)]
impl Stream_WriteScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}
