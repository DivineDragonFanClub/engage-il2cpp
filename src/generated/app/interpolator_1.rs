
use crate::app::interpolatortime::IInterpolatorTime;
use crate::app::interpolatortime::InterpolatorTime;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolator_1/Interpolator_1.md")))]
#[::unity2::class(namespace = "App", name = "Interpolator`1")]
pub struct Interpolator_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Prev")]
    pub m_prev: T0,
    #[rename(name = "m_Next")]
    pub m_next: T0,
}

#[cfg(feature = "app-interpolator_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Interpolator_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> T0;

    #[method(name = "get_Prev", args = 0)]
    pub fn get_prev(self) -> T0;

    #[method(name = "set_Prev", args = 1)]
    pub fn set_prev(self, value: T0) -> ();

    #[method(name = "get_Next", args = 0)]
    pub fn get_next(self) -> T0;

    #[method(name = "set_Next", args = 1)]
    pub fn set_next(self, value: T0) -> ();

    #[method(name = "get_Goal", args = 0)]
    pub fn get_goal(self) -> T0;

    #[method(name = "IsEqual", args = 2)]
    pub fn is_equal(self, a: T0, b: T0) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set(self, value: T0, time: f32) -> bool;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(v: crate::app::interpolator_1::Interpolator_1<T0>) -> T0;
}

#[cfg(feature = "app-interpolator_1")]
impl<T0: ::unity2::ClassIdentity> Interpolator_1<T0> {
    pub fn new(r#type: crate::app::curve::Curve_Type, num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Interpolator_1),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolator_1Methods<T0>>::ctor(this, r#type, num);
        this
    }
}
