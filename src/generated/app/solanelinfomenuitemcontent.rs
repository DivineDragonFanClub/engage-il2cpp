
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenuitemcontent/SolanelInfoMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct SolanelInfoMenuItemContent {
    #[rename(name = "m_IconObjList")]
    pub m_icon_obj_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_SubNameObj")]
    pub m_sub_name_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
}

#[cfg(feature = "app-solanelinfomenuitemcontent")]
#[::unity2::methods]
impl SolanelInfoMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-solanelinfomenuitemcontent")]
impl SolanelInfoMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuItemContentMethods>::ctor(this);
        this
    }
}
