
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonscriptableobject_1/SingletonScriptableObject_1.md")))]
#[::unity2::class(namespace = "App", name = "SingletonScriptableObject`1")]
pub struct SingletonScriptableObject_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
}

#[cfg(feature = "app-singletonscriptableobject_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SingletonScriptableObject_1<T0> {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-singletonscriptableobject_1")]
impl<T0: ::unity2::ClassIdentity> SingletonScriptableObject_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonScriptableObject_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonScriptableObject_1Methods<T0>>::ctor(this);
        this
    }
}
