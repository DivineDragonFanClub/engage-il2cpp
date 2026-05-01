
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/reliancepopupcontroller/ReliancePopUpController.md")))]
#[::unity2::class(namespace = "App", name = "ReliancePopUpController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ReliancePopUpController {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_UnitIcon0")]
    pub m_unit_icon0: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_UnitIcon1")]
    pub m_unit_icon1: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_Heart0")]
    pub m_heart0: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Heart1")]
    pub m_heart1: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WdwReliance")]
    pub m_wdw_reliance: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WdwTalk")]
    pub m_wdw_talk: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TextReliance")]
    pub m_text_reliance: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TextTalk")]
    pub m_text_talk: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TalkIcon")]
    pub m_talk_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-reliancepopupcontroller")]
#[::unity2::methods]
impl ReliancePopUpController {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::reliancepopupcontroller::ReliancePopUpController;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: crate::app::reliancepopupcontroller::ReliancePopUpController) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "IsPlayingAnim", args = 0)]
    pub fn is_playing_anim(self) -> bool;

    #[method(name = "PopUp", args = 3)]
    pub fn pop_up(
        self,
        unit_l: crate::app::unit::Unit,
        unit_r: crate::app::unit::Unit,
        value: i32,
    ) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "ActiveReliance", args = 0)]
    pub fn active_reliance(self) -> ();

    #[method(name = "ActiveTalk", args = 0)]
    pub fn active_talk(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-reliancepopupcontroller")]
impl ReliancePopUpController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReliancePopUpController),
                ::core::stringify!(new),
            )
        });
        <Self as IReliancePopUpControllerMethods>::ctor(this);
        this
    }
}
