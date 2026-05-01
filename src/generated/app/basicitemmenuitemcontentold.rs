
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicitemmenuitemcontentold/BasicItemMenuItemContentOld.md")))]
#[::unity2::class(namespace = "App", name = "BasicItemMenuItemContentOld")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct BasicItemMenuItemContentOld {
    #[rename(name = "m_icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_text")]
    pub m_text: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_count")]
    pub m_count: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-basicitemmenuitemcontentold")]
#[::unity2::methods]
impl BasicItemMenuItemContentOld {
    #[method(name = "SetupObject", args = 0)]
    pub fn setup_object(self) -> ();

    #[method(name = "Blank", args = 0)]
    pub fn blank(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "GetTextComponent", args = 0)]
    pub fn get_text_component(self) -> crate::unity_engine::ui::text::Text;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-basicitemmenuitemcontentold")]
impl BasicItemMenuItemContentOld {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicItemMenuItemContentOld),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicItemMenuItemContentOldMethods>::ctor(this);
        this
    }
}
