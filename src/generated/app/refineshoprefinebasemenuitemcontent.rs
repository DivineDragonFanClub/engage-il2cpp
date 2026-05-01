
use crate::app::basicitemmenuitemcontent::BasicItemMenuItemContent;
use crate::app::basicitemmenuitemcontent::IBasicItemMenuItemContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinebasemenuitemcontent/RefineShopRefineBaseMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineBaseMenuItemContent")]
#[parent(crate::app::basicitemmenuitemcontent::BasicItemMenuItemContent)]
pub struct RefineShopRefineBaseMenuItemContent {
    #[rename(name = "m_EmptyMessageText")]
    pub m_empty_message_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineshoprefinebasemenuitemcontent")]
#[::unity2::methods]
impl RefineShopRefineBaseMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshoprefinebasemenuitemcontent")]
impl RefineShopRefineBaseMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineBaseMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineBaseMenuItemContentMethods>::ctor(this);
        this
    }
}
