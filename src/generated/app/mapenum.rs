
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapenum/MapEnum_AreaEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapEnum_AreaEnumerator {
    pub m_current: crate::app::mappos::MapPos,
    pub m_min_x: i32,
    pub m_min_z: i32,
    pub m_max_x: i32,
    pub m_max_z: i32,
}

impl ::unity2::ClassIdentity for MapEnum_AreaEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapEnum.AreaEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapEnum_AreaEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapenum")]
#[::unity2::methods(value)]
impl MapEnum_AreaEnumerator {
    #[method(name = "Setup", args = 4)]
    pub fn setup(
        self,
        x: i32,
        z: i32,
        w: i32,
        h: i32,
    ) -> crate::app::mapenum::MapEnum_AreaEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::mappos::MapPos;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::mapenum::MapEnum_AreaEnumerator;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapenum/MapEnum_RangeEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapEnum_RangeEnumerator {
    pub m_current: crate::app::maprange::MapRange,
    pub m_pivot_x: i32,
    pub m_pivot_z: i32,
    pub m_min_x: i32,
    pub m_min_z: i32,
    pub m_max_x: i32,
    pub m_max_z: i32,
    pub m_near: i32,
    pub m_far: i32,
}

impl ::unity2::ClassIdentity for MapEnum_RangeEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapEnum.RangeEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapEnum_RangeEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapenum")]
#[::unity2::methods(value)]
impl MapEnum_RangeEnumerator {
    #[method(name = "Setup", args = 4)]
    pub fn setup(
        self,
        x: i32,
        z: i32,
        near: i32,
        far: i32,
    ) -> crate::app::mapenum::MapEnum_RangeEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::maprange::MapRange;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::mapenum::MapEnum_RangeEnumerator;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapenum/MapEnum.md")))]
#[::unity2::class(namespace = "App", name = "MapEnum")]
#[parent(crate::system::object::Object)]
pub struct MapEnum {
    #[static_field]
    #[rename(name = "s_AreaEnumerator")]
    pub s_area_enumerator: crate::app::mapenum::MapEnum_AreaEnumerator,
    #[static_field]
    #[rename(name = "s_RangeEnumerator")]
    pub s_range_enumerator: crate::app::mapenum::MapEnum_RangeEnumerator,
    #[static_field]
    #[rename(name = "s_MoveEnumerator")]
    pub s_move_enumerator: crate::app::mapenum::MapEnum_MoveEnumerator,
    #[static_field]
    #[rename(name = "s_CellEnumerator")]
    pub s_cell_enumerator: crate::app::mapenum::MapEnum_CellEnumerator,
}

#[cfg(feature = "app-mapenum")]
#[::unity2::methods]
impl MapEnum {
    #[method(name = "GetFull", args = 0)]
    pub fn get_full() -> crate::app::mapenum::MapEnum_AreaEnumerator;

    #[method(name = "GetPlayArea", args = 0)]
    pub fn get_play_area() -> crate::app::mapenum::MapEnum_AreaEnumerator;

    #[method(name = "GetArea", args = 4)]
    pub fn get_area(x: i32, z: i32, w: i32, h: i32) -> crate::app::mapenum::MapEnum_AreaEnumerator;

    #[method(name = "GetRange", args = 3)]
    pub fn get_range(x: i32, z: i32, range: i32) -> crate::app::mapenum::MapEnum_RangeEnumerator;

    #[method(name = "GetRange", args = 4)]
    pub fn get_range_2(
        x: i32,
        z: i32,
        near: i32,
        far: i32,
    ) -> crate::app::mapenum::MapEnum_RangeEnumerator;

    #[method(name = "GetAround", args = 2)]
    pub fn get_around(x: i32, z: i32) -> crate::app::mapenum::MapEnum_RangeEnumerator;

    #[method(name = "GetMove", args = 0)]
    pub fn get_move() -> crate::app::mapenum::MapEnum_MoveEnumerator;

    #[method(name = "GetCell", args = 1)]
    pub fn get_cell(unit: crate::app::unit::Unit) -> crate::app::mapenum::MapEnum_CellEnumerator;

    #[method(name = "GetCell", args = 3)]
    pub fn get_cell_2(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
    ) -> crate::app::mapenum::MapEnum_CellEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapenum")]
impl MapEnum {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEnum),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEnumMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapenum/MapEnum_MoveEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapEnum_MoveEnumerator {
    pub m_current: crate::app::maprange::MapRange,
    pub m_min_x: i32,
    pub m_min_z: i32,
    pub m_max_x: i32,
    pub m_max_z: i32,
}

impl ::unity2::ClassIdentity for MapEnum_MoveEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapEnum.MoveEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapEnum_MoveEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapenum")]
#[::unity2::methods(value)]
impl MapEnum_MoveEnumerator {
    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> crate::app::mapenum::MapEnum_MoveEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::maprange::MapRange;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::mapenum::MapEnum_MoveEnumerator;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapenum/MapEnum_CellEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapEnum_CellEnumerator {
    pub m_current: crate::app::mappos::MapPos,
    pub m_x: i32,
    pub m_z: i32,
    pub m_size: i32,
}

impl ::unity2::ClassIdentity for MapEnum_CellEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapEnum.CellEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapEnum_CellEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapenum")]
#[::unity2::methods(value)]
impl MapEnum_CellEnumerator {
    #[method(name = "Setup", args = 3)]
    pub fn setup(self, x: i32, z: i32, size: i32) -> crate::app::mapenum::MapEnum_CellEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::mappos::MapPos;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::mapenum::MapEnum_CellEnumerator;
}
