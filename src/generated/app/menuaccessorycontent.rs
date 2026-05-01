
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menuaccessorycontent/MenuAccessoryContent.md")))]
#[::unity2::class(namespace = "App", name = "MenuAccessoryContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MenuAccessoryContent {
    #[rename(name = "m_MasterMenu")]
    pub m_master_menu: crate::app::basicmenu::BasicMenu,
    #[rename(name = "m_winAnimator")]
    pub m_win_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-menuaccessorycontent")]
#[::unity2::methods]
impl MenuAccessoryContent {
    #[method(name = "GetMenu", args = 0)]
    pub fn get_menu(self) -> crate::app::basicmenu::BasicMenu;

    #[method(name = "SetMenu", args = 1)]
    pub fn set_menu(self, menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OpenAnime", args = 0)]
    pub fn open_anime(self) -> ();

    #[method(name = "CloseAnime", args = 0)]
    pub fn close_anime(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-menuaccessorycontent")]
impl MenuAccessoryContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MenuAccessoryContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMenuAccessoryContentMethods>::ctor(this);
        this
    }
}
