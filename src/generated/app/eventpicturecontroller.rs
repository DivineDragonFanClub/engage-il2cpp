
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventpicturecontroller/EventPictureController.md")))]
#[::unity2::class(namespace = "App", name = "EventPictureController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EventPictureController {
    #[static_field]
    #[rename(name = "AssetRootPath")]
    pub asset_root_path: ::unity2::Il2CppString,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::rawimage::RawImage,
    #[rename(name = "m_Animation")]
    pub m_animation: crate::unity_engine::animation::Animation,
    #[rename(name = "m_PictureTextureHandle")]
    pub m_picture_texture_handle: crate::app::resourcehandle_2::ResourceHandle_2,
}

#[cfg(feature = "app-eventpicturecontroller")]
#[::unity2::methods]
impl EventPictureController {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "IsSkip", args = 0)]
    pub fn is_skip(self) -> bool;

    #[method(name = "IsLoadingTexture", args = 0)]
    pub fn is_loading_texture(self) -> bool;

    #[method(name = "Show", args = 3)]
    pub fn show(
        self,
        texture_name: ::unity2::Il2CppString,
        anim_name: ::unity2::Il2CppString,
        is_female: bool,
    ) -> ();

    #[method(name = "Hide", args = 1)]
    pub fn hide(self, anim_name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-eventpicturecontroller")]
impl EventPictureController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventPictureController),
                ::core::stringify!(new),
            )
        });
        <Self as IEventPictureControllerMethods>::ctor(this);
        this
    }
}
