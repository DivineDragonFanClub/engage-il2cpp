
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenuitemcontent/ProfileCardAlbumListMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardAlbumListMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ProfileCardAlbumListMenuItemContent {
    #[rename(name = "m_Title")]
    pub m_title: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BackImage")]
    pub m_back_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_IsInitialized")]
    pub m_is_initialized: bool,
}

#[cfg(feature = "app-profilecardalbumlistmenuitemcontent")]
#[::unity2::methods]
impl ProfileCardAlbumListMenuItemContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetGameObjects", args = 0)]
    pub fn set_game_objects(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "SetActiveChildren", args = 1)]
    pub fn set_active_children(self, is_active: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardalbumlistmenuitemcontent")]
impl ProfileCardAlbumListMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenuItemContentMethods>::ctor(this);
        this
    }
}
