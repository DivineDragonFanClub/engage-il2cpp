
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/interpolatortime/InterpolatorTime.md")))]
#[::unity2::class(namespace = "App", name = "InterpolatorTime")]
#[parent(crate::system::object::Object)]
pub struct InterpolatorTime {
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Tick")]
    pub m_tick: f32,
    #[rename(name = "m_Type")]
    pub m_type: crate::app::curve::Curve_Type,
    #[rename(name = "m_Num")]
    pub m_num: i32,
    #[rename(name = "m_IsFirst")]
    pub m_is_first: bool,
    #[rename(name = "m_IsDirty")]
    pub m_is_dirty: bool,
}

#[cfg(feature = "app-interpolatortime")]
#[::unity2::methods]
impl InterpolatorTime {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "get_Time", args = 0)]
    pub fn get_time(self) -> f32;

    #[method(name = "get_IsFirst", args = 0)]
    pub fn get_is_first(self) -> bool;

    #[method(name = "get_IsStability", args = 0)]
    pub fn get_is_stability(self) -> bool;

    #[method(name = "get_IsNextStability", args = 0)]
    pub fn get_is_next_stability(self) -> bool;

    #[method(name = "get_IsRunning", args = 0)]
    pub fn get_is_running(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Instant", args = 0)]
    pub fn instant(self) -> ();

    #[method(name = "SetCurve", args = 2)]
    pub fn set_curve(self, r#type: crate::app::curve::Curve_Type, num: i32) -> ();

    #[method(name = "SetCurve", args = 1)]
    pub fn set_curve_2(self, r#type: crate::app::curve::Curve_Type) -> ();

    #[method(name = "GetRate", args = 0)]
    pub fn get_rate(self) -> f32;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(self, prev: f32, next: f32) -> f32;

    #[method(name = "SetTime", args = 1)]
    pub fn set_time(self, time: f32) -> ();

    #[method(name = "get_NextTick", args = 0)]
    pub fn get_next_tick(self) -> f32;

    #[method(name = "Tick", args = 1)]
    pub fn tick(self, is_fade_skip: bool) -> bool;
}

#[cfg(feature = "app-interpolatortime")]
impl InterpolatorTime {
    pub fn new(r#type: crate::app::curve::Curve_Type, num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpolatorTime),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpolatorTimeMethods>::ctor(this, r#type, num);
        this
    }
}
