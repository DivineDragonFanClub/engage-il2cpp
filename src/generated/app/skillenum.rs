
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillenum/SkillEnum_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SkillEnum_Enumerator {
    pub m_index: i32,
    pub m_count: i32,
    pub m_array: crate::app::skillarray::SkillArray,
    pub m_current: crate::app::skilldata::SkillData,
    pub m_mask: crate::app::skilldata::SkillData_TimingMasks,
}

impl ::unity2::ClassIdentity for SkillEnum_Enumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SkillEnum.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SkillEnum_Enumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-skillenum")]
#[::unity2::methods(value)]
impl SkillEnum_Enumerator {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        array: crate::app::skillarray::SkillArray,
        mask: crate::app::skilldata::SkillData_TimingMasks,
        count: i32,
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
    pub fn get_current(self) -> crate::app::skilldata::SkillData;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillenum/SkillEnum.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SkillEnum {
    pub m_array: crate::app::skillarray::SkillArray,
    pub m_mask: crate::app::skilldata::SkillData_TimingMasks,
}

impl ::unity2::ClassIdentity for SkillEnum {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SkillEnum";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SkillEnum {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-skillenum")]
#[::unity2::methods(value)]
impl SkillEnum {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        array: crate::app::skillarray::SkillArray,
        mask: crate::app::skilldata::SkillData_TimingMasks,
    ) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(
        array: crate::app::skillarray::SkillArray,
        mask: crate::app::skilldata::SkillData_TimingMasks,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "Get", args = 2)]
    pub fn get_2(
        unit: crate::app::unit::Unit,
        mask: crate::app::skilldata::SkillData_TimingMasks,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "Get", args = 2)]
    pub fn get_3(
        array: crate::app::skillarray::SkillArray,
        timing: crate::app::skilldata::SkillData_Timings,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "Get", args = 2)]
    pub fn get_4(
        unit: crate::app::unit::Unit,
        timing: crate::app::skilldata::SkillData_Timings,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "GetSupports", args = 1)]
    pub fn get_supports(
        array: crate::app::skillarray::SkillArray,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "GetSupports", args = 1)]
    pub fn get_supports_2(unit: crate::app::unit::Unit) -> crate::app::skillenum::SkillEnum;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::skillenum::SkillEnum_Enumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetUnsafeList", args = 0)]
    pub fn get_unsafe_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::skilldata::SkillData>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
