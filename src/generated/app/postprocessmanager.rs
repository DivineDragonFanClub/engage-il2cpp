
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/postprocessmanager/PostProcessManager.md")))]
#[::unity2::class(namespace = "App", name = "PostProcessManager")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: postprocessmanager :: PostProcessManager >)]
pub struct PostProcessManager {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::rendering::volume::Volume,
    #[rename(name = "m_Bmap")]
    pub m_bmap: crate::unity_engine::rendering::volume::Volume,
    #[rename(name = "m_Combat")]
    pub m_combat: crate::unity_engine::rendering::volume::Volume,
    #[rename(name = "BmapCombatChangeTime")]
    pub bmap_combat_change_time: f32,
    #[rename(name = "CurveInterpolate")]
    pub curve_interpolate: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "CurveBlur")]
    pub curve_blur: crate::unity_engine::animationcurve::AnimationCurve,
}

#[cfg(feature = "app-postprocessmanager")]
#[::unity2::methods]
impl PostProcessManager {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "TryAddCustomRadialBlur", args = 0)]
    pub fn try_add_custom_radial_blur(self) -> ();

    #[method(name = "SetWeight", args = 2)]
    pub fn set_weight(
        self,
        volume: crate::unity_engine::rendering::volume::Volume,
        weight: f32,
    ) -> ();

    #[method(name = "SetEnabled", args = 2)]
    pub fn set_enabled(
        self,
        volume: crate::unity_engine::rendering::volume::Volume,
        enabled: bool,
    ) -> ();

    #[method(name = "SetLayer", args = 2)]
    pub fn set_layer(
        self,
        volume: crate::unity_engine::rendering::volume::Volume,
        layer: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetVolume", args = 1)]
    pub fn get_volume(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::rendering::volume::Volume;

    #[method(name = "SetTransition", args = 1)]
    pub fn set_transition(self, transition: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-postprocessmanager")]
impl PostProcessManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PostProcessManager),
                ::core::stringify!(new),
            )
        });
        <Self as IPostProcessManagerMethods>::ctor(this);
        this
    }
}
