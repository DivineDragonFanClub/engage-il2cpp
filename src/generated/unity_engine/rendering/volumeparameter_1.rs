
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::volumeparameter::IVolumeParameter;
use crate::unity_engine::rendering::volumeparameter::VolumeParameter;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumeparameter_1/VolumeParameter_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "VolumeParameter`1")]
pub struct VolumeParameter_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Value")]
    pub m_value: T0,
}

#[cfg(feature = "unity_engine-rendering-volumeparameter_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> VolumeParameter_1<T0> {
    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> T0;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: T0) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, value: T0, override_state: bool) -> ();

    #[method(name = "Interp", args = 3)]
    pub fn interp(
        self,
        from: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        to: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        t: f32,
    ) -> ();

    #[method(name = "Interp", args = 3)]
    pub fn interp_2(self, from: T0, to: T0, t: f32) -> ();

    #[method(name = "Override", args = 1)]
    pub fn r#override(self, x: T0) -> ();

    #[method(name = "SetValue", args = 1)]
    pub fn set_value_2(
        self,
        parameter: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1<T0>,
        rhs: T0,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1<T0>,
        rhs: T0,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1<T0>,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "op_Explicit", args = 1)]
    pub fn op_explicit(
        prop: crate::unity_engine::rendering::volumeparameter_1::VolumeParameter_1<T0>,
    ) -> T0;
}

#[cfg(feature = "unity_engine-rendering-volumeparameter_1")]
impl<T0: ::unity2::ClassIdentity> VolumeParameter_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeParameter_1),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeParameter_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(value: T0, override_state: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeParameter_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as IVolumeParameter_1Methods<T0>>::ctor_2(this, value, override_state);
        this
    }
}
