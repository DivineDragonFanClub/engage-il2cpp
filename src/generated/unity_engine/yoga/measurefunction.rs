
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/yoga/measurefunction/MeasureFunction.md")))]
#[::unity2::class(namespace = "UnityEngine.Yoga", name = "MeasureFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MeasureFunction {}

#[cfg(feature = "unity_engine-yoga-measurefunction")]
#[::unity2::methods]
impl MeasureFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 5)]
    pub fn invoke(
        self,
        node: crate::unity_engine::yoga::yoganode::YogaNode,
        width: f32,
        width_mode: crate::unity_engine::yoga::yogameasuremode::YogaMeasureMode,
        height: f32,
        height_mode: crate::unity_engine::yoga::yogameasuremode::YogaMeasureMode,
    ) -> crate::unity_engine::yoga::yogasize::YogaSize;
}

#[cfg(feature = "unity_engine-yoga-measurefunction")]
impl MeasureFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MeasureFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IMeasureFunctionMethods>::ctor(this, object, method);
        this
    }
}
