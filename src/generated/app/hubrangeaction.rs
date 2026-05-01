
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubrangeaction/HubRangeAction.md")))]
#[::unity2::class(namespace = "App", name = "HubRangeAction")]
#[parent(crate::system::object::Object)]
pub struct HubRangeAction {
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_triggerIn")]
    pub m_trigger_in: bool,
    #[rename(name = "m_triggerOut")]
    pub m_trigger_out: bool,
    #[rename(name = "m_funcInRange")]
    pub m_func_in_range:
        crate::system::action_1::Action_1<crate::app::hubunitcontroller::HubUnitController>,
    #[rename(name = "m_funcOutRange")]
    pub m_func_out_range:
        crate::system::action_1::Action_1<crate::app::hubunitcontroller::HubUnitController>,
}

#[cfg(feature = "app-hubrangeaction")]
#[::unity2::methods]
impl HubRangeAction {
    #[method(name = "get_IsStop", args = 0)]
    pub fn get_is_stop(self) -> bool;

    #[method(name = "set_IsStop", args = 1)]
    pub fn set_is_stop(self, value: bool) -> ();

    #[method(name = "get_Distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_Distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_Interval", args = 0)]
    pub fn get_interval(self) -> f32;

    #[method(name = "set_Interval", args = 1)]
    pub fn set_interval(self, value: f32) -> ();

    #[method(name = "get_Angle", args = 0)]
    pub fn get_angle(self) -> f32;

    #[method(name = "set_Angle", args = 1)]
    pub fn set_angle(self, value: f32) -> ();

    #[method(name = "get_IsStopMain", args = 0)]
    pub fn get_is_stop_main(self) -> bool;

    #[method(name = "set_IsStopMain", args = 1)]
    pub fn set_is_stop_main(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        func_in_range: crate::system::action_1::Action_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
        func_out_range: crate::system::action_1::Action_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
    ) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, unit: crate::app::hubunitcontroller::HubUnitController) -> ();
}

#[cfg(feature = "app-hubrangeaction")]
impl HubRangeAction {
    pub fn new(
        func_in_range: crate::system::action_1::Action_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
        func_out_range: crate::system::action_1::Action_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubRangeAction),
                ::core::stringify!(new),
            )
        });
        <Self as IHubRangeActionMethods>::ctor(this, func_in_range, func_out_range);
        this
    }
}
