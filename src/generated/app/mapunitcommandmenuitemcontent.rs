
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenuitemcontent/MapUnitCommandMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MapUnitCommandMenuItemContent {
    #[rename(name = "m_Title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Help")]
    pub m_help: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-mapunitcommandmenuitemcontent")]
#[::unity2::methods]
impl MapUnitCommandMenuItemContent {
    #[method(name = "FindGameObject", args = 1)]
    pub fn find_game_object(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetupObjects", args = 0)]
    pub fn setup_objects(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetHelpText", args = 2)]
    pub fn set_help_text(
        self,
        help: ::unity2::Il2CppString,
        c: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetHelpTextAlpha", args = 1)]
    pub fn set_help_text_alpha(self, alpha: f32) -> ();

    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenuitemcontent")]
impl MapUnitCommandMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenuItemContentMethods>::ctor(this);
        this
    }
}
