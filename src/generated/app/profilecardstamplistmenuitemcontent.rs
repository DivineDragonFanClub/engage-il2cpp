
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenuitemcontent/ProfileCardStampListMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampListMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ProfileCardStampListMenuItemContent {
    #[rename(name = "m_StampImage")]
    pub m_stamp_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NewIconObject")]
    pub m_new_icon_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-profilecardstamplistmenuitemcontent")]
#[::unity2::methods]
impl ProfileCardStampListMenuItemContent {
    #[method(name = "SetStampImage", args = 1)]
    pub fn set_stamp_image(
        self,
        stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
    ) -> ();

    #[method(name = "SetStampColor", args = 1)]
    pub fn set_stamp_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetNewIconActive", args = 1)]
    pub fn set_new_icon_active(self, active: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardstamplistmenuitemcontent")]
impl ProfileCardStampListMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenuItemContentMethods>::ctor(this);
        this
    }
}
