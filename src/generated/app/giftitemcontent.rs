
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/giftitemcontent/GiftItemContent.md")))]
#[::unity2::class(namespace = "App", name = "GiftItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct GiftItemContent {
    #[rename(name = "m_Name")]
    pub m_name: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Count")]
    pub m_count: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-giftitemcontent")]
#[::unity2::methods]
impl GiftItemContent {
    #[method(name = "SetupObjects", args = 0)]
    pub fn setup_objects(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-giftitemcontent")]
impl GiftItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GiftItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGiftItemContentMethods>::ctor(this);
        this
    }
}
