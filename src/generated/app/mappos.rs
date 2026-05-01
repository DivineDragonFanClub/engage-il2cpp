
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappos/MapPos.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapPos {
    pub x: i32,
    pub z: i32,
}

impl ::unity2::ClassIdentity for MapPos {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapPos";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapPos {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mappos")]
#[::unity2::methods(value)]
impl MapPos {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, other: crate::app::mappos::MapPos) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, x: i32, z: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, other: crate::app::mappos::MapPos) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(self, x: i32, z: i32) -> ();

    #[method(name = "AddX", args = 1)]
    pub fn add_x(self, x: i32) -> ();

    #[method(name = "AddZ", args = 1)]
    pub fn add_z(self, z: i32) -> ();

    #[method(name = "Sub", args = 1)]
    pub fn sub(self, other: crate::app::mappos::MapPos) -> ();

    #[method(name = "Sub", args = 2)]
    pub fn sub_2(self, x: i32, z: i32) -> ();

    #[method(name = "SubX", args = 1)]
    pub fn sub_x(self, x: i32) -> ();

    #[method(name = "SubZ", args = 1)]
    pub fn sub_z(self, z: i32) -> ();

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        lhs: crate::app::mappos::MapPos,
        rhs: crate::app::mappos::MapPos,
    ) -> crate::app::mappos::MapPos;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        lhs: crate::app::mappos::MapPos,
        rhs: crate::app::mappos::MapPos,
    ) -> crate::app::mappos::MapPos;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(lhs: crate::app::mappos::MapPos, rhs: crate::app::mappos::MapPos) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(lhs: crate::app::mappos::MapPos, rhs: crate::app::mappos::MapPos) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, rhs_obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, rhs: crate::app::mappos::MapPos) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
