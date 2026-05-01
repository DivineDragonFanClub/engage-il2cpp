
use crate::app::interpolator_1::IInterpolator_1;
use crate::app::interpolator_1::Interpolator_1;
use crate::app::interpolatortime::IInterpolatorTime;
use crate::app::interpolatortime::InterpolatorTime;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolatorfloat/InterpolatorFloat.md")))]
#[::unity2::class(namespace = "App", name = "InterpolatorFloat")]
# [parent (crate :: app :: interpolator_1 :: Interpolator_1 < f32 >)]
pub struct InterpolatorFloat {}

#[cfg(feature = "app-interpolatorfloat")]
#[::unity2::methods]
impl InterpolatorFloat {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "IsEqual", args = 2)]
    pub fn is_equal(self, a: f32, b: f32) -> bool;
}

#[cfg(feature = "app-interpolatorfloat")]
impl InterpolatorFloat {
    pub fn new(r#type: crate::app::curve::Curve_Type, num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpolatorFloat),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolatorFloatMethods>::ctor(this, r#type, num);
        this
    }
}
