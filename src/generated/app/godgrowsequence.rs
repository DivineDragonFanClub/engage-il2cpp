
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godgrowsequence/GodGrowSequence.md")))]
#[::unity2::class(namespace = "App", name = "GodGrowSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GodGrowSequence {
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Exp")]
    pub m_exp: i32,
    #[rename(name = "m_Dirty")]
    pub m_dirty: i32,
    #[rename(name = "m_OldLevel")]
    pub m_old_level: i32,
}

#[cfg(feature = "app-godgrowsequence")]
#[::unity2::methods]
impl GodGrowSequence {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        exp: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        exp: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind_3(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        exp: i32,
        dirty: i32,
    ) -> ();

    #[method(name = "CreateBindImpl", args = 5)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        exp: i32,
        dirty: i32,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        exp: i32,
        dirty: i32,
    ) -> ();

    #[method(name = "GainExp", args = 0)]
    pub fn gain_exp(self) -> ();

    #[method(name = "GainDirty", args = 0)]
    pub fn gain_dirty(self) -> ();

    #[method(name = "CheckNotifyLevelCapTalk", args = 0)]
    pub fn check_notify_level_cap_talk(self) -> ();

    #[method(name = "NotifyLevelCapTalk", args = 0)]
    pub fn notify_level_cap_talk(self) -> ();

    #[method(name = "CheckLevelUp", args = 0)]
    pub fn check_level_up(self) -> ();

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();
}

#[cfg(feature = "app-godgrowsequence")]
impl GodGrowSequence {
    pub fn new(
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        exp: i32,
        dirty: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodGrowSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGodGrowSequenceMethods>::ctor(this, god_unit, unit, exp, dirty);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godgrowsequence/GodGrowSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodGrowSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodGrowSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodGrowSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodGrowSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodGrowSequence_Label {
    pub fn level_up() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
