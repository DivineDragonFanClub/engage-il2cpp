
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsoundsequence/MyRoomSoundSequence.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSoundSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: myroomsoundsequence :: MyRoomSoundSequence >)]
pub struct MyRoomSoundSequence {}

#[cfg(feature = "app-myroomsoundsequence")]
#[::unity2::methods]
impl MyRoomSoundSequence {
    #[method(name = "get_IsJukebox", args = 0)]
    pub fn get_is_jukebox(self) -> bool;

    #[method(name = "set_IsJukebox", args = 1)]
    pub fn set_is_jukebox(self, value: bool) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, is_jukebox: bool) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateSoundMenu", args = 0)]
    pub fn create_sound_menu(self) -> ();

    #[method(name = "Entry", args = 0)]
    pub fn entry(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomsoundsequence")]
impl MyRoomSoundSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSoundSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSoundSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsoundsequence/MyRoomSoundSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomSoundSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomSoundSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomSoundSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomSoundSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomSoundSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn main() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}
