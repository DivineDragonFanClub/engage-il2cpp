
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillinheritancesequence/SkillInheritanceSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SkillInheritanceSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SkillInheritanceSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SkillInheritanceSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SkillInheritanceSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SkillInheritanceSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn unit_select() -> Self {
        Self { value: 1 }
    }

    pub fn skill_select() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillinheritancesequence/SkillInheritanceSequence.md")))]
#[::unity2::class(namespace = "App", name = "SkillInheritanceSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct SkillInheritanceSequence {}

#[cfg(feature = "app-skillinheritancesequence")]
#[::unity2::methods]
impl SkillInheritanceSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "CreateUnitSelectMenu", args = 0)]
    pub fn create_unit_select_menu(self) -> ();

    #[method(name = "CreateSkillInheritanceMenu", args = 0)]
    pub fn create_skill_inheritance_menu(self) -> ();

    #[method(name = "InheritanceStart", args = 0)]
    pub fn inheritance_start(self) -> ();

    #[method(name = "InheritanceEnd", args = 0)]
    pub fn inheritance_end(self) -> ();

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "get_SelectUnit", args = 0)]
    pub fn get_select_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_SelectUnit", args = 1)]
    pub fn set_select_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_SelectUnitGodList", args = 0)]
    pub fn get_select_unit_god_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>;

    #[method(name = "set_SelectUnitGodList", args = 1)]
    pub fn set_select_unit_god_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    ) -> ();
}

#[cfg(feature = "app-skillinheritancesequence")]
impl SkillInheritanceSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillInheritanceSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillInheritanceSequenceMethods>::ctor(this);
        this
    }
}
