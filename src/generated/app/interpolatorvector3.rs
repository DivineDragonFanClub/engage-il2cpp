
use crate::app::interpolator_1::IInterpolator_1;
use crate::app::interpolator_1::Interpolator_1;
use crate::app::interpolatortime::IInterpolatorTime;
use crate::app::interpolatortime::InterpolatorTime;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolatorvector3/InterpolatorVector3.md")))]
#[::unity2::class(namespace = "App", name = "InterpolatorVector3")]
# [parent (crate :: app :: interpolator_1 :: Interpolator_1 < crate :: unity_engine :: vector3 :: Vector3 >)]
pub struct InterpolatorVector3 {}

#[cfg(feature = "app-interpolatorvector3")]
#[::unity2::methods]
impl InterpolatorVector3 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "IsEqual", args = 2)]
    pub fn is_equal(
        self,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> bool;
}

#[cfg(feature = "app-interpolatorvector3")]
impl InterpolatorVector3 {
    pub fn new(r#type: crate::app::curve::Curve_Type, num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpolatorVector3),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolatorVector3Methods>::ctor(this, r#type, num);
        this
    }
}
