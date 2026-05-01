
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedatahandle/SaveDataHandle_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SaveDataHandle_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SaveDataHandle_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SaveDataHandle.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SaveDataHandle_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SaveDataHandle_States {
    pub fn running() -> Self {
        Self { value: 0 }
    }

    pub fn succeeded() -> Self {
        Self { value: 1 }
    }

    pub fn failed() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedatahandle/SaveDataHandle.md")))]
#[::unity2::class(namespace = "App", name = "SaveDataHandle")]
#[parent(crate::system::object::Object)]
pub struct SaveDataHandle {}

#[cfg(feature = "app-savedatahandle")]
#[::unity2::methods]
impl SaveDataHandle {
    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "IsFinished", args = 0)]
    pub fn is_finished(self) -> bool;

    #[method(name = "IsSucceeded", args = 0)]
    pub fn is_succeeded(self) -> bool;

    #[method(name = "IsFailed", args = 0)]
    pub fn is_failed(self) -> bool;

    #[method(name = "get_State", args = 0)]
    pub fn get_state(self) -> crate::app::savedatahandle::SaveDataHandle_States;

    #[method(name = "set_State", args = 1)]
    pub fn set_state(self, value: crate::app::savedatahandle::SaveDataHandle_States) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-savedatahandle")]
impl SaveDataHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SaveDataHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISaveDataHandleMethods>::ctor(this);
        this
    }
}
