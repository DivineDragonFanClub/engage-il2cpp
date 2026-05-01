
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleparam/BattleParam.md")))]
#[::unity2::class(namespace = "App", name = "BattleParam")]
#[parent(crate::system::object::Object)]
pub struct BattleParam {
    #[static_field]
    #[rename(name = "INVALID")]
    pub invalid: f32,
    #[static_field]
    #[rename(name = "Mins")]
    pub mins: ::unity2::Array<f32>,
    #[static_field]
    #[rename(name = "Maxs")]
    pub maxs: ::unity2::Array<f32>,
    #[static_field]
    #[rename(name = "Clamps")]
    pub clamps: ::unity2::Array<f32>,
    #[rename(name = "Add")]
    pub add: f32,
    #[rename(name = "Scale")]
    pub scale: f32,
    #[rename(name = "Value")]
    pub value: f32,
}

#[cfg(feature = "app-battleparam")]
#[::unity2::methods]
impl BattleParam {
    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::battleparam::BattleParam_Kinds;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetResult", args = 1)]
    pub fn get_result(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "Calculate", args = 1)]
    pub fn calculate(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, src: crate::app::battleparam::BattleParam) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::battleparam::BattleParam) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-battleparam")]
impl BattleParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleParam),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleParamMethods>::ctor(this);
        this
    }

    pub fn new_2(src: crate::app::battleparam::BattleParam) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleParam),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBattleParamMethods>::ctor_2(this, src);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleparam/BattleParam_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleParam_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleParam_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleParam.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleParam_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleParam_Kinds {
    pub fn value() -> Self {
        Self { value: 0 }
    }

    pub fn ratio() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }
}
