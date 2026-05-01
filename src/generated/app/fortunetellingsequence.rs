
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fortunetellingsequence/FortuneTellingSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FortuneTellingSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FortuneTellingSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FortuneTellingSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FortuneTellingSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FortuneTellingSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn unit_select() -> Self {
        Self { value: 1 }
    }

    pub fn fortune_telling() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fortunetellingsequence/FortuneTellingSequence.md")))]
#[::unity2::class(namespace = "App", name = "FortuneTellingSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct FortuneTellingSequence {}

#[cfg(feature = "app-fortunetellingsequence")]
#[::unity2::methods]
impl FortuneTellingSequence {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateUnitSelectMenu", args = 0)]
    pub fn create_unit_select_menu(self) -> ();

    #[method(name = "Result", args = 0)]
    pub fn result(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "get_SelectUnit", args = 0)]
    pub fn get_select_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_SelectUnit", args = 1)]
    pub fn set_select_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Seadas", args = 0)]
    pub fn get_seadas(self) -> crate::combat::character::Character;

    #[method(name = "set_Seadas", args = 1)]
    pub fn set_seadas(self, value: crate::combat::character::Character) -> ();
}

#[cfg(feature = "app-fortunetellingsequence")]
impl FortuneTellingSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FortuneTellingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IFortuneTellingSequenceMethods>::ctor(this);
        this
    }
}
