
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaninggaugecontroller/RingCleaningGaugeController.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningGaugeController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingCleaningGaugeController {
    #[rename(name = "GaugeObject")]
    pub gauge_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "DirtyTextureCurve")]
    pub dirty_texture_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "MaxGaugeWidth")]
    pub max_gauge_width: f32,
    #[rename(name = "m_InterpolatorWidth")]
    pub m_interpolator_width: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_ConditionIcon")]
    pub m_condition_icon: crate::app::ringcleaningconditionicon::RingCleaningConditionIcon,
}

#[cfg(feature = "app-ringcleaninggaugecontroller")]
#[::unity2::methods]
impl RingCleaningGaugeController {
    #[method(name = "get_DirtyRate", args = 0)]
    pub fn get_dirty_rate(self) -> f32;

    #[method(name = "get_IsFinishCleaning", args = 0)]
    pub fn get_is_finish_cleaning(self) -> bool;

    #[method(name = "set_IsFinishCleaning", args = 1)]
    pub fn set_is_finish_cleaning(self, value: bool) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, dirty: i32) -> ();

    #[method(name = "ChangeDirty", args = 2)]
    pub fn change_dirty(self, old_dirty: i32, new_dirty: i32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "SetDirty", args = 1)]
    pub fn set_dirty(self, dirty: i32) -> ();

    #[method(name = "GetDirtyTextureValue", args = 1)]
    pub fn get_dirty_texture_value(self, value: f32) -> f32;

    #[method(name = "GetGuageWidth", args = 1)]
    pub fn get_guage_width(self, dirty: i32) -> f32;

    #[method(name = "InitGaugeWidth", args = 1)]
    pub fn init_gauge_width(self, dirty: i32) -> ();

    #[method(name = "SetConditionIcon", args = 1)]
    pub fn set_condition_icon(self, rect: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "UpdateGaugeWidth", args = 0)]
    pub fn update_gauge_width(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringcleaninggaugecontroller")]
impl RingCleaningGaugeController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningGaugeController),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningGaugeControllerMethods>::ctor(this);
        this
    }
}
