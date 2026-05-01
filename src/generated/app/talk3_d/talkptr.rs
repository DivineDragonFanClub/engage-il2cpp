
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkptr/TalkPtr.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkPtr")]
#[parent(crate::system::object::Object)]
pub struct TalkPtr {
    #[static_field]
    #[rename(name = "CharSize")]
    pub char_size: i32,
    #[static_field]
    #[rename(name = "GroupSize")]
    pub group_size: i32,
}

#[cfg(feature = "app-talk3_d-talkptr")]
#[::unity2::methods]
impl TalkPtr {
    #[method(name = "get_OriginalPtr", args = 0)]
    pub fn get_original_ptr(self) -> ::unity2::IntPtr;

    #[method(name = "set_OriginalPtr", args = 1)]
    pub fn set_original_ptr(self, value: ::unity2::IntPtr) -> ();

    #[method(name = "get_NowPtr", args = 0)]
    pub fn get_now_ptr(self) -> ::unity2::IntPtr;

    #[method(name = "set_NowPtr", args = 1)]
    pub fn set_now_ptr(self, value: ::unity2::IntPtr) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, ptr: ::unity2::IntPtr) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "ResetPosition", args = 0)]
    pub fn reset_position(self) -> ();

    #[method(name = "Forward", args = 1)]
    pub fn forward(self, size: i32) -> ();

    #[method(name = "ReadTagGroup", args = 0)]
    pub fn read_tag_group(self) -> crate::app::mess::Mess_TagGroup;

    #[method(name = "ReadInt32", args = 0)]
    pub fn read_int32(self) -> i32;

    #[method(name = "ReadInt16", args = 0)]
    pub fn read_int16(self) -> i32;

    #[method(name = "ReadByte", args = 0)]
    pub fn read_byte(self) -> i32;

    #[method(name = "ReadChar", args = 0)]
    pub fn read_char(self) -> u16;

    #[method(name = "ReadStringParam", args = 0)]
    pub fn read_string_param(self) -> ::unity2::Il2CppString;

    #[method(name = "ReadPID", args = 0)]
    pub fn read_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "WriteShiftIn", args = 0)]
    pub fn write_shift_in(self) -> ();

    #[method(name = "WriteShiftOut", args = 0)]
    pub fn write_shift_out(self) -> ();

    #[method(name = "WriteTagGroup", args = 1)]
    pub fn write_tag_group(self, group: crate::app::mess::Mess_TagGroup) -> ();

    #[method(name = "WriteInt16", args = 1)]
    pub fn write_int16(self, value: i16) -> ();

    #[method(name = "WriteInt16", args = 1)]
    pub fn write_int16_2(self, value: i32) -> ();

    #[method(name = "WriteString", args = 1)]
    pub fn write_string(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "IsNull", args = 0)]
    pub fn is_null(self) -> bool;
}

#[cfg(feature = "app-talk3_d-talkptr")]
impl TalkPtr {
    pub fn new(ptr: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkPtr),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkPtrMethods>::ctor(this, ptr);
        this
    }

    pub fn new_2(mid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkPtr),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITalkPtrMethods>::ctor_2(this, mid);
        this
    }
}
