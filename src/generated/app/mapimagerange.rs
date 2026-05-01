
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagerange/MapImageRange.md")))]
#[::unity2::class(namespace = "App", name = "MapImageRange")]
#[parent(crate::system::object::Object)]
pub struct MapImageRange {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Image")]
    pub m_image: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_Poss")]
    pub m_poss: crate::system::collections::generic::list_1::List_1<
        crate::app::mapimagerange::MapImageRange_Pos,
    >,
}

#[cfg(feature = "app-mapimagerange")]
#[::unity2::methods]
impl MapImageRange {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, x: i32, z: i32) -> ();

    #[method(name = "Delete", args = 2)]
    pub fn delete(self, x: i32, z: i32) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(self, x: i32, z: i32) -> bool;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "GetX", args = 1)]
    pub fn get_x(self, i: i32) -> i32;

    #[method(name = "GetZ", args = 1)]
    pub fn get_z(self, i: i32) -> i32;

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "OpenFan", args = 5)]
    pub fn open_fan(
        self,
        start_x: i32,
        start_z: i32,
        target_x: i32,
        target_z: i32,
        angle: i32,
    ) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimagerange")]
impl MapImageRange {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageRange),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageRangeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagerange/MapImageRange_Pos.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapImageRange_Pos {
    pub x: u8,
    pub z: u8,
}

impl ::unity2::ClassIdentity for MapImageRange_Pos {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapImageRange.Pos";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapImageRange_Pos {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapimagerange")]
#[::unity2::methods(value)]
impl MapImageRange_Pos {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, x: i32, z: i32) -> ();
}
