
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/scalefunc/ScaleFunc.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ScaleFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ScaleFunc {}

#[cfg(feature = "unity_engine-rendering-scalefunc")]
#[::unity2::methods]
impl ScaleFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        size: crate::unity_engine::vector2int::Vector2Int,
    ) -> crate::unity_engine::vector2int::Vector2Int;
}

#[cfg(feature = "unity_engine-rendering-scalefunc")]
impl ScaleFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScaleFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IScaleFuncMethods>::ctor(this, object, method);
        this
    }
}
