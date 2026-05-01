
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolatorrotationcurve/InterpolatorRotationCurve.md")))]
#[::unity2::class(namespace = "App", name = "InterpolatorRotationCurve")]
#[parent(crate::system::object::Object)]
pub struct InterpolatorRotationCurve {
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_term")]
    pub m_term: f32,
}

#[cfg(feature = "app-interpolatorrotationcurve")]
#[::unity2::methods]
impl InterpolatorRotationCurve {
    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: f32) -> ();

    #[method(name = "get_Prev", args = 0)]
    pub fn get_prev(self) -> f32;

    #[method(name = "set_Prev", args = 1)]
    pub fn set_prev(self, value: f32) -> ();

    #[method(name = "get_Next", args = 0)]
    pub fn get_next(self) -> f32;

    #[method(name = "set_Next", args = 1)]
    pub fn set_next(self, value: f32) -> ();

    #[method(name = "get_IsEnd", args = 0)]
    pub fn get_is_end(self) -> bool;

    #[method(name = "get_Curve", args = 0)]
    pub fn get_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, animation_curve: crate::unity_engine::animationcurve::AnimationCurve) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, value: f32, time: f32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-interpolatorrotationcurve")]
impl InterpolatorRotationCurve {
    pub fn new(animation_curve: crate::unity_engine::animationcurve::AnimationCurve) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpolatorRotationCurve),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolatorRotationCurveMethods>::ctor(this, animation_curve);
        this
    }
}
