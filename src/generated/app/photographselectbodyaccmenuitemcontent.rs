
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectbodyaccmenuitemcontent/PhotographSelectBodyAccMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectBodyAccMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct PhotographSelectBodyAccMenuItemContent {
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CursorObj")]
    pub m_cursor_obj: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-photographselectbodyaccmenuitemcontent")]
#[::unity2::methods]
impl PhotographSelectBodyAccMenuItemContent {
    #[method(name = "BuildObject", args = 0)]
    pub fn build_object(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographselectbodyaccmenuitemcontent")]
impl PhotographSelectBodyAccMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectBodyAccMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectBodyAccMenuItemContentMethods>::ctor(this);
        this
    }
}
