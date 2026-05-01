
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingcharaimagerender/FishingCharaImageRender.md")))]
#[::unity2::class(namespace = "App", name = "FishingCharaImageRender")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FishingCharaImageRender {
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_Chara")]
    pub m_chara: crate::combat::character::Character,
    #[rename(name = "m_Sola")]
    pub m_sola: crate::combat::character::Character,
    #[rename(name = "m_Lure")]
    pub m_lure: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsPlayingHitAnime")]
    pub m_is_playing_hit_anime: bool,
}

#[cfg(feature = "app-fishingcharaimagerender")]
#[::unity2::methods]
impl FishingCharaImageRender {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetLayer", args = 2)]
    pub fn set_layer(self, obj: crate::unity_engine::gameobject::GameObject, set_layer: i32) -> ();

    #[method(name = "PlayCharaAnime", args = 3)]
    pub fn play_chara_anime(
        self,
        name: ::unity2::Il2CppString,
        face_state: ::unity2::Il2CppString,
        change_soon: bool,
    ) -> ();

    #[method(name = "StartHitAnime", args = 0)]
    pub fn start_hit_anime(self) -> ();

    #[method(name = "StartAssistAnime", args = 0)]
    pub fn start_assist_anime(self) -> ();

    #[method(name = "Init", args = 1)]
    pub fn init(self, rod_id: ::unity2::Il2CppString) -> ();

    #[method(name = "ChangeRodType", args = 1)]
    pub fn change_rod_type(self, rod_id: ::unity2::Il2CppString) -> ();

    #[method(name = "SetSolaVisible", args = 1)]
    pub fn set_sola_visible(self, set: bool) -> ();

    #[method(name = "PlaySolaAnime", args = 1)]
    pub fn play_sola_anime(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-fishingcharaimagerender")]
impl FishingCharaImageRender {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingCharaImageRender),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingCharaImageRenderMethods>::ctor(this);
        this
    }
}
