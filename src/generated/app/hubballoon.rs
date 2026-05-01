
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubballoon/HubBalloon.md")))]
#[::unity2::class(namespace = "App", name = "HubBalloon")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubBalloon {
    #[rename(name = "m_propetyToID")]
    pub m_propety_to_id: i32,
    #[rename(name = "m_talkDefault")]
    pub m_talk_default: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_talkChapter")]
    pub m_talk_chapter: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_talkReliance")]
    pub m_talk_reliance: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_rendererList")]
    pub m_renderer_list: ::unity2::Array<crate::unity_engine::renderer::Renderer>,
    #[rename(name = "m_materials")]
    pub m_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_alpha")]
    pub m_alpha: f32,
    #[rename(name = "m_talkDefaultActive")]
    pub m_talk_default_active: bool,
    #[rename(name = "m_talkChapterActive")]
    pub m_talk_chapter_active: bool,
    #[rename(name = "m_talkRelianceActive")]
    pub m_talk_reliance_active: bool,
}

#[cfg(feature = "app-hubballoon")]
#[::unity2::methods]
impl HubBalloon {
    #[method(name = "get_BalloonType", args = 0)]
    pub fn get_balloon_type(self) -> crate::app::hubballoon::HubBalloon_Type;

    #[method(name = "set_BalloonType", args = 1)]
    pub fn set_balloon_type(self, value: crate::app::hubballoon::HubBalloon_Type) -> ();

    #[method(name = "get_UseUpIcon", args = 0)]
    pub fn get_use_up_icon(self) -> bool;

    #[method(name = "set_UseUpIcon", args = 1)]
    pub fn set_use_up_icon(self, value: bool) -> ();

    #[method(name = "get_TargetAccess", args = 0)]
    pub fn get_target_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "set_TargetAccess", args = 1)]
    pub fn set_target_access(self, value: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "get_FadeDistance", args = 0)]
    pub fn get_fade_distance(self) -> f32;

    #[method(name = "set_FadeDistance", args = 1)]
    pub fn set_fade_distance(self, value: f32) -> ();

    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "set_PlayerController", args = 1)]
    pub fn set_player_controller(
        self,
        value: crate::app::hubplayercontroller::HubPlayerController,
    ) -> ();

    #[method(name = "SetActiveTalkDefault", args = 1)]
    pub fn set_active_talk_default(self, value: bool) -> ();

    #[method(name = "SetActiveTalkChapter", args = 1)]
    pub fn set_active_talk_chapter(self, value: bool) -> ();

    #[method(name = "SetActiveTalkReliance", args = 1)]
    pub fn set_active_talk_reliance(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsOutRange", args = 0)]
    pub fn is_out_range(self) -> bool;

    #[method(name = "IsFadeAlpha", args = 0)]
    pub fn is_fade_alpha(self) -> bool;

    #[method(name = "IsActiveAccess", args = 0)]
    pub fn is_active_access(self) -> bool;

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "UpdateAlpha", args = 1)]
    pub fn update_alpha(self, gain: f32) -> ();

    #[method(name = "SetMaterialDark", args = 0)]
    pub fn set_material_dark(self) -> ();

    #[method(name = "SetMaterialLight", args = 0)]
    pub fn set_material_light(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubballoon")]
impl HubBalloon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubBalloon),
                ::core::stringify!(new),
            )
        });
        <Self as IHubBalloonMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubballoon/HubBalloon_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubBalloon_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubBalloon_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubBalloon.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubBalloon_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubBalloon_Type {
    pub fn default() -> Self {
        Self { value: 0 }
    }

    pub fn chapter() -> Self {
        Self { value: 1 }
    }

    pub fn reliance() -> Self {
        Self { value: 2 }
    }
}
