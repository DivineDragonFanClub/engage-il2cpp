
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/performdynamicres/PerformDynamicRes.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "PerformDynamicRes")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct PerformDynamicRes {}

#[cfg(feature = "unity_engine-rendering-performdynamicres")]
#[::unity2::methods]
impl PerformDynamicRes {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> f32;
}

#[cfg(feature = "unity_engine-rendering-performdynamicres")]
impl PerformDynamicRes {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PerformDynamicRes),
                ::core::stringify!(new),
            )
        });
        <Self as IPerformDynamicResMethods>::ctor(this, object, method);
        this
    }
}
