
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenum/UnitEnum.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct UnitEnum {
    pub m_types: ::unity2::Array<crate::app::force::Force_Type>,
}

impl ::unity2::ClassIdentity for UnitEnum {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitEnum";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitEnum {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-unitenum")]
#[::unity2::methods(value)]
impl UnitEnum {
    #[method(name = "GetEnumerator", args = 1)]
    pub fn get_enumerator(
        types: ::unity2::Array<crate::app::force::Force_Type>,
    ) -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetOnMapForce", args = 0)]
    pub fn get_on_map_force() -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetEnemy", args = 1)]
    pub fn get_enemy(force: crate::app::force::Force_Type) -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetAlly", args = 1)]
    pub fn get_ally(force: crate::app::force::Force_Type) -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetMask", args = 1)]
    pub fn get_mask(mask: u32) -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetOnSortieForce", args = 0)]
    pub fn get_on_sortie_force() -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetUsedForce", args = 0)]
    pub fn get_used_force() -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetSelfForce", args = 0)]
    pub fn get_self_force() -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetSamePlayerForce", args = 0)]
    pub fn get_same_player_force() -> crate::app::unitenum::UnitEnum;

    #[method(name = "Get", args = 1)]
    pub fn get(r#type: crate::app::force::Force_Type) -> crate::app::unitenum::UnitEnum;

    #[method(name = "GetTarget", args = 3)]
    pub fn get_target(
        unit: crate::app::unit::Unit,
        near: i32,
        far: i32,
    ) -> crate::app::unitenum::UnitEnum_TargetEnumerator;

    #[method(name = "GetTarget", args = 2)]
    pub fn get_target_2(
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::unitenum::UnitEnum_TargetEnumerator;

    #[method(name = "GetTarget", args = 5)]
    pub fn get_target_3(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        near: i32,
        far: i32,
    ) -> crate::app::unitenum::UnitEnum_TargetEnumerator;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, types: ::unity2::Array<crate::app::force::Force_Type>) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator_2(self) -> crate::app::unitenum::UnitEnum_Enumerator;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenum/UnitEnum_TargetEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct UnitEnum_TargetEnumerator {
    pub m_unit_enum: crate::app::unitenum::UnitEnum_Enumerator,
    pub m_current: crate::app::unit::Unit,
    pub m_unit: crate::app::unit::Unit,
    pub m_x: i32,
    pub m_z: i32,
    pub m_near: i32,
    pub m_far: i32,
}

impl ::unity2::ClassIdentity for UnitEnum_TargetEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitEnum.TargetEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitEnum_TargetEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-unitenum")]
#[::unity2::methods(value)]
impl UnitEnum_TargetEnumerator {
    #[method(name = "Setup", args = 5)]
    pub fn setup(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        near: i32,
        far: i32,
    ) -> crate::app::unitenum::UnitEnum_TargetEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::unit::Unit;

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
    pub fn get_enumerator(self) -> crate::app::unitenum::UnitEnum_TargetEnumerator;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenum/UnitEnum_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct UnitEnum_Enumerator {
    pub m_types: ::unity2::Array<crate::app::force::Force_Type>,
    pub m_index: i32,
    pub m_next: crate::app::unit::Unit,
    pub m_current: crate::app::unit::Unit,
}

impl ::unity2::ClassIdentity for UnitEnum_Enumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitEnum.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitEnum_Enumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-unitenum")]
#[::unity2::methods(value)]
impl UnitEnum_Enumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, types: ::unity2::Array<crate::app::force::Force_Type>) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::unit::Unit;
}
