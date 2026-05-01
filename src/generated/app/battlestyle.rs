
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlestyle/BattleStyle_Types.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleStyle_Types {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleStyle_Types {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleStyle.Types";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleStyle_Types {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleStyle_Types {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn cooperation() -> Self {
        Self { value: 1 }
    }

    pub fn horse() -> Self {
        Self { value: 2 }
    }

    pub fn covert() -> Self {
        Self { value: 3 }
    }

    pub fn heavy() -> Self {
        Self { value: 4 }
    }

    pub fn fly() -> Self {
        Self { value: 5 }
    }

    pub fn magic() -> Self {
        Self { value: 6 }
    }

    pub fn prana() -> Self {
        Self { value: 7 }
    }

    pub fn dragon() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlestyle/BattleStyle.md")))]
#[::unity2::class(namespace = "App", name = "BattleStyle")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: battlestyle :: BattleStyle >)]
pub struct BattleStyle {
    #[static_field]
    #[rename(name = "Begin")]
    pub begin: crate::app::battlestyle::BattleStyle_Types,
    #[static_field]
    #[rename(name = "End")]
    pub end: crate::app::battlestyle::BattleStyle_Types,
    #[static_field]
    #[rename(name = "Count")]
    pub count: i32,
    #[static_field]
    #[rename(name = "Names")]
    pub names: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-battlestyle")]
#[::unity2::methods]
impl BattleStyle {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style(name: ::unity2::Il2CppString) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = "GetName", args = 1)]
    pub fn get_name(style: crate::app::battlestyle::BattleStyle_Types) -> ::unity2::Il2CppString;

    #[method(name = "GetSkills", args = 1)]
    pub fn get_skills(
        style: crate::app::battlestyle::BattleStyle_Types,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetStyleName", args = 1)]
    pub fn get_style_name(
        style: crate::app::battlestyle::BattleStyle_Types,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 1)]
    pub fn get_help(style: crate::app::battlestyle::BattleStyle_Types) -> ::unity2::Il2CppString;

    #[method(name = "get_Style", args = 0)]
    pub fn get_style_2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Style", args = 1)]
    pub fn set_style(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name_2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help_2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Skills", args = 0)]
    pub fn get_skills_2(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Skills", args = 1)]
    pub fn set_skills(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-battlestyle")]
impl BattleStyle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleStyle),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleStyleMethods>::ctor(this);
        this
    }
}
