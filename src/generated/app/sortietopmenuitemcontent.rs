
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietopmenuitemcontent/SortieTopMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "SortieTopMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct SortieTopMenuItemContent {
    #[rename(name = "m_Partner")]
    pub m_partner: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_help")]
    pub m_help: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-sortietopmenuitemcontent")]
#[::unity2::methods]
impl SortieTopMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "SetHelpText", args = 2)]
    pub fn set_help_text(
        self,
        help: ::unity2::Il2CppString,
        c: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetHelpActive", args = 2)]
    pub fn set_help_active(self, b_active: bool, c: crate::unity_engine::color::Color) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortietopmenuitemcontent")]
impl SortieTopMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTopMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTopMenuItemContentMethods>::ctor(this);
        this
    }
}
