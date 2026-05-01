
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkstatemonitor/TalkStateMonitor.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkStateMonitor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TalkStateMonitor {
    #[rename(name = "m_CharaName")]
    pub m_chara_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextPid")]
    pub m_text_pid: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextPause")]
    pub m_text_pause: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextFace")]
    pub m_text_face: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TextLook")]
    pub m_text_look: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_OffstY")]
    pub m_offst_y: i32,
}

#[cfg(feature = "app-talk3_d-talkstatemonitor")]
#[::unity2::methods]
impl TalkStateMonitor {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "AdjustPosition", args = 1)]
    pub fn adjust_position(self, position: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "AdjustPosition", args = 2)]
    pub fn adjust_position_2(
        self,
        camera: crate::unity_engine::camera::Camera,
        chara: crate::combat::character::Character,
    ) -> ();

    #[method(name = "SetCharaName", args = 1)]
    pub fn set_chara_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetPid", args = 1)]
    pub fn set_pid(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetBodyAnime", args = 1)]
    pub fn set_body_anime(self, anime_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetFaceAnime", args = 1)]
    pub fn set_face_anime(self, anime_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetLookAt", args = 1)]
    pub fn set_look_at(self, locator_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkstatemonitor")]
impl TalkStateMonitor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkStateMonitor),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkStateMonitorMethods>::ctor(this);
        this
    }
}
