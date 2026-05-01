
use crate::app::interpolator_1::IInterpolator_1;
use crate::app::interpolator_1::Interpolator_1;
use crate::app::interpolatortime::IInterpolatorTime;
use crate::app::interpolatortime::InterpolatorTime;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolatorint/InterpolatorInt.md")))]
#[::unity2::class(namespace = "App", name = "InterpolatorInt")]
# [parent (crate :: app :: interpolator_1 :: Interpolator_1 < i32 >)]
pub struct InterpolatorInt {}

#[cfg(feature = "app-interpolatorint")]
#[::unity2::methods]
impl InterpolatorInt {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "IsEqual", args = 2)]
    pub fn is_equal(self, a: i32, b: i32) -> bool;
}

#[cfg(feature = "app-interpolatorint")]
impl InterpolatorInt {
    pub fn new(r#type: crate::app::curve::Curve_Type, num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpolatorInt),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolatorIntMethods>::ctor(this, r#type, num);
        this
    }
}
