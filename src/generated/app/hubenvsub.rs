
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubenvsub/HubEnvSub.md")))]
#[::unity2::class(namespace = "App", name = "HubEnvSub")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubEnvSub {
    #[rename(name = "m_EditTimezoneType")]
    pub m_edit_timezone_type: crate::app::hubutil::HubUtil_TimezoneType,
    #[rename(name = "m_MorningFogColor")]
    pub m_morning_fog_color: crate::unity_engine::color::Color,
    #[rename(name = "m_MorningFogStart")]
    pub m_morning_fog_start: f32,
    #[rename(name = "m_MorningFogEnd")]
    pub m_morning_fog_end: f32,
    #[rename(name = "m_DayFogColor")]
    pub m_day_fog_color: crate::unity_engine::color::Color,
    #[rename(name = "m_DayFogStart")]
    pub m_day_fog_start: f32,
    #[rename(name = "m_DayFogEnd")]
    pub m_day_fog_end: f32,
    #[rename(name = "m_EveningFogColor")]
    pub m_evening_fog_color: crate::unity_engine::color::Color,
    #[rename(name = "m_EveningFogStart")]
    pub m_evening_fog_start: f32,
    #[rename(name = "m_EveningFogEnd")]
    pub m_evening_fog_end: f32,
    #[rename(name = "m_NightFogColor")]
    pub m_night_fog_color: crate::unity_engine::color::Color,
    #[rename(name = "m_NightFogStart")]
    pub m_night_fog_start: f32,
    #[rename(name = "m_NightFogEnd")]
    pub m_night_fog_end: f32,
    #[rename(name = "m_Morning")]
    pub m_morning: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Day")]
    pub m_day: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Evening")]
    pub m_evening: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Night")]
    pub m_night: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TimezoneType")]
    pub m_timezone_type: crate::app::hubutil::HubUtil_TimezoneType,
    #[rename(name = "m_PrevDepth")]
    pub m_prev_depth: i32,
    #[rename(name = "m_PrevFogColor")]
    pub m_prev_fog_color: crate::unity_engine::color::Color,
    #[rename(name = "m_PrevFogStart")]
    pub m_prev_fog_start: f32,
    #[rename(name = "m_PrevFogEnd")]
    pub m_prev_fog_end: f32,
}

#[cfg(feature = "app-hubenvsub")]
#[::unity2::methods]
impl HubEnvSub {
    #[method(name = "get_Morning", args = 0)]
    pub fn get_morning(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_Day", args = 0)]
    pub fn get_day(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_Evening", args = 0)]
    pub fn get_evening(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_Night", args = 0)]
    pub fn get_night(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PushParams", args = 1)]
    pub fn push_params(self, timezone_type: crate::app::hubutil::HubUtil_TimezoneType) -> ();

    #[method(name = "PopParams", args = 0)]
    pub fn pop_params(self) -> ();

    #[method(name = "ApplyFog", args = 0)]
    pub fn apply_fog(self) -> ();

    #[method(name = "SupportFogStack", args = 0)]
    pub fn support_fog_stack(self) -> bool;

    #[method(name = "SetPostProcessActive", args = 2)]
    pub fn set_post_process_active(
        self,
        obj: crate::unity_engine::gameobject::GameObject,
        value: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubenvsub")]
impl HubEnvSub {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubEnvSub),
                ::core::stringify!(new),
            )
        });
        <Self as IHubEnvSubMethods>::ctor(this);
        this
    }
}
