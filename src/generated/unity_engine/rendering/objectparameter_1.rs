
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use crate::unity_engine::rendering::volumeparameter_1::IVolumeParameter_1;
use crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/objectparameter_1/ObjectParameter_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ObjectParameter`1")]
pub struct ObjectParameter_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-rendering-objectparameter_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ObjectParameter_1<T0> {
    #[method(name = "get_parameters", args = 0)]
    pub fn get_parameters(
        self,
    ) -> crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
        crate::unity_engine::rendering::volumeparameter::VolumeParameter,
    >;

    #[method(name = "set_parameters", args = 1)]
    pub fn set_parameters(
        self,
        value: crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
            crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        >,
    ) -> ();

    #[method(name = "get_overrideState", args = 0)]
    pub fn get_override_state(self) -> bool;

    #[method(name = "set_overrideState", args = 1)]
    pub fn set_override_state(self, value: bool) -> ();

    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> T0;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: T0) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: T0) -> ();

    #[method(name = "Interp", args = 3)]
    pub fn interp(
        self,
        from: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        to: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        t: f32,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-objectparameter_1")]
impl<T0: ::unity2::ClassIdentity> ObjectParameter_1<T0> {
    pub fn new(value: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObjectParameter_1),
                ::core::stringify!(new),
            )
        });
        <Self as IObjectParameter_1Methods<T0>>::ctor(this, value);
        this
    }
}
