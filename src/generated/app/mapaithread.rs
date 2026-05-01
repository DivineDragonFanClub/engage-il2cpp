
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaithread/MapAiThread_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapAiThread_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapAiThread_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapAiThread.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapAiThread_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapAiThread_Status {
    pub fn wait() -> Self {
        Self { value: 0 }
    }

    pub fn run() -> Self {
        Self { value: 1 }
    }

    pub fn finalize() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaithread/MapAiThread.md")))]
#[::unity2::class(namespace = "App", name = "MapAiThread")]
#[parent(crate::system::object::Object)]
pub struct MapAiThread {
    #[rename(name = "m_Lock")]
    pub m_lock: ::unity2::IlInstance,
    #[rename(name = "m_IsFirstRun")]
    pub m_is_first_run: bool,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::mapaithread::MapAiThread_Status,
}

#[cfg(feature = "app-mapaithread")]
#[::unity2::methods]
impl MapAiThread {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Destruct", args = 0)]
    pub fn destruct(self) -> ();

    #[method(name = "Function", args = 1)]
    pub fn function(obj: crate::system::object::Object) -> ();

    #[method(name = "SetFirstRunFlag", args = 0)]
    pub fn set_first_run_flag(self) -> ();

    #[method(name = "Run", args = 0)]
    pub fn run(self) -> ();

    #[method(name = "IsWait", args = 0)]
    pub fn is_wait(self) -> bool;
}

#[cfg(feature = "app-mapaithread")]
impl MapAiThread {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAiThread),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAiThreadMethods>::ctor(this);
        this
    }
}
