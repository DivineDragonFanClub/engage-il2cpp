
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/baseinvokablecall/BaseInvokableCall.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "BaseInvokableCall")]
#[parent(crate::system::object::Object)]
pub struct BaseInvokableCall {}

#[cfg(feature = "unity_engine-events-baseinvokablecall")]
#[::unity2::methods]
impl BaseInvokableCall {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        target: crate::system::object::Object,
        function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, args: ::unity2::Array<crate::system::object::Object>) -> ();

    #[method(name = "AllowInvoke", args = 1)]
    pub fn allow_invoke(delegate: crate::system::delegate::Delegate) -> bool;

    #[method(name = "Find", args = 2)]
    pub fn find(
        self,
        target_obj: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> bool;
}

#[cfg(feature = "unity_engine-events-baseinvokablecall")]
impl BaseInvokableCall {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseInvokableCall),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseInvokableCallMethods>::ctor(this);
        this
    }

    pub fn new_2(
        target: crate::system::object::Object,
        function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseInvokableCall),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBaseInvokableCallMethods>::ctor_2(this, target, function);
        this
    }
}
