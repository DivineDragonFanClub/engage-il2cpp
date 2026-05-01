
use crate::app::mapimagecore_1::IMapImageCore_1;
use crate::app::mapimagecore_1::MapImageCore_1;
use crate::app::mapimagecorebyte::IMapImageCoreByte;
use crate::app::mapimagecorebyte::MapImageCoreByte;
use crate::app::mapimageindex::IMapImageIndex;
use crate::app::mapimageindex::MapImageIndex;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageunit/MapImageUnit_PositionScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapImageUnit_PositionScope {
    pub m_unit: crate::app::unit::Unit,
    pub m_x: i32,
    pub m_z: i32,
}

impl ::unity2::ClassIdentity for MapImageUnit_PositionScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapImageUnit.PositionScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapImageUnit_PositionScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapimageunit")]
#[::unity2::methods(value)]
impl MapImageUnit_PositionScope {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageunit/MapImageUnit_UnitScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapImageUnit_UnitScope {
    pub m_unit: crate::app::unit::Unit,
}

impl ::unity2::ClassIdentity for MapImageUnit_UnitScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapImageUnit.UnitScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapImageUnit_UnitScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapimageunit")]
#[::unity2::methods(value)]
impl MapImageUnit_UnitScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageunit/MapImageUnit.md")))]
#[::unity2::class(namespace = "App", name = "MapImageUnit")]
#[parent(crate::app::mapimagecorebyte::MapImageCoreByte)]
pub struct MapImageUnit {
    #[rename(name = "m_Cells")]
    pub m_cells: ::unity2::Array<crate::app::mappos::MapPos>,
}

#[cfg(feature = "app-mapimageunit")]
#[::unity2::methods]
impl MapImageUnit {
    #[method(name = "GetUnit", args = 2)]
    pub fn get_unit(self, x: i32, z: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnitWithMind", args = 2)]
    pub fn get_unit_with_mind(self, x: i32, z: i32) -> crate::app::unit::Unit;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "TryDelete", args = 1)]
    pub fn try_delete(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "UpdateSupportSkill", args = 0)]
    pub fn update_support_skill(self) -> ();

    #[method(name = "UpdateSupportSkill", args = 3)]
    pub fn update_support_skill_2(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "AddImpl", args = 3)]
    pub fn add_impl(self, unit: crate::app::unit::Unit, base_x: i32, base_z: i32) -> ();

    #[method(name = "DeleteImpl", args = 3)]
    pub fn delete_impl(self, unit: crate::app::unit::Unit, base_x: i32, base_z: i32) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_2(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "AddNoUpdatingSupportSkill", args = 3)]
    pub fn add_no_updating_support_skill(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Delete", args = 3)]
    pub fn delete_2(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "DeleteNoUpdatingSupportSkill", args = 3)]
    pub fn delete_no_updating_support_skill(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "TryMove", args = 5)]
    pub fn try_move(
        self,
        unit: crate::app::unit::Unit,
        old_x: i32,
        old_z: i32,
        new_x: i32,
        new_z: i32,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimageunit")]
impl MapImageUnit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageUnit),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageUnitMethods>::ctor(this);
        this
    }
}
