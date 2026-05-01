
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamemessagecontent/GameMessageContent.md")))]
#[::unity2::class(namespace = "App", name = "GameMessageContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GameMessageContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_objText")]
    pub m_obj_text: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objKeyWaitIcon")]
    pub m_obj_key_wait_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objLoadingIcon")]
    pub m_obj_loading_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_proc")]
    pub m_proc: crate::app::gamemessage::GameMessage,
    #[rename(name = "m_posX")]
    pub m_pos_x: f32,
    #[rename(name = "m_posY")]
    pub m_pos_y: f32,
}

#[cfg(feature = "app-gamemessagecontent")]
#[::unity2::methods]
impl GameMessageContent {
    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "SetProc", args = 1)]
    pub fn set_proc(self, proc: crate::app::gamemessage::GameMessage) -> ();

    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "CalcPosX", args = 0)]
    pub fn calc_pos_x(self) -> f32;

    #[method(name = "CalcPosY", args = 0)]
    pub fn calc_pos_y(self) -> f32;

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "SetShadowOff", args = 0)]
    pub fn set_shadow_off(self) -> ();

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, x: f32, y: f32) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::gamemessagecontent::GameMessageContent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gamemessagecontent")]
impl GameMessageContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameMessageContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGameMessageContentMethods>::ctor(this);
        this
    }
}
