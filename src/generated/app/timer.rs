
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/timer/Timer.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Timer {}

impl ::unity2::ClassIdentity for Timer {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Timer";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Timer {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-timer")]
#[::unity2::methods(value)]
impl Timer {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        is_show: bool,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Start", args = 3)]
    pub fn start(
        self,
        is_show: bool,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Start", args = 2)]
    pub fn start_2(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "get_DeltaTime", args = 0)]
    pub fn get_delta_time(self) -> f32;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;
}
