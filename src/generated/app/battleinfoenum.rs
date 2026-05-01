
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleinfoenum/BattleInfoEnum.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BattleInfoEnum {
    pub m_info: crate::app::battleinfo::BattleInfo,
    pub m_min: crate::app::battleside::BattleSide_Type,
    pub m_max: crate::app::battleside::BattleSide_Type,
    pub m_current: crate::app::battleinfoside::BattleInfoSide,
}

impl ::unity2::ClassIdentity for BattleInfoEnum {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleInfoEnum";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleInfoEnum {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-battleinfoenum")]
#[::unity2::methods(value)]
impl BattleInfoEnum {
    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::battleinfoenum::BattleInfoEnum;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        info: crate::app::battleinfo::BattleInfo,
        min: crate::app::battleside::BattleSide_Type,
        max: crate::app::battleside::BattleSide_Type,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::battleinfoside::BattleInfoSide;
}
