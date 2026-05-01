
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingeventfader/FishingEventFader.md")))]
#[::unity2::class(namespace = "App", name = "FishingEventFader")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FishingEventFader {
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Layer")]
    pub m_layer: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Timer")]
    pub m_timer: f32,
    #[rename(name = "m_FinishTime")]
    pub m_finish_time: f32,
    #[rename(name = "IsFadeIn")]
    pub is_fade_in: bool,
    #[rename(name = "m_FadeTimeList")]
    pub m_fade_time_list: ::unity2::Array<f32>,
}

#[cfg(feature = "app-fishingeventfader")]
#[::unity2::methods]
impl FishingEventFader {
    #[method(name = "get_IsRun", args = 0)]
    pub fn get_is_run(self) -> bool;

    #[method(name = "set_IsRun", args = 1)]
    pub fn set_is_run(self, value: bool) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "FadeIn", args = 1)]
    pub fn fade_in(self, r#type: crate::app::fishing::fadetype::FadeType) -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(self, r#type: crate::app::fishing::fadetype::FadeType) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, set: crate::unity_engine::color::Color) -> ();

    #[method(name = "GetFadeTime", args = 1)]
    pub fn get_fade_time(self, r#type: crate::app::fishing::fadetype::FadeType) -> f32;

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishingeventfader")]
impl FishingEventFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingEventFader),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingEventFaderMethods>::ctor(this);
        this
    }
}
