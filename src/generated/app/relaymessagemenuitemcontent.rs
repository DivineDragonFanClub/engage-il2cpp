
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymessagemenuitemcontent/RelayMessageMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RelayMessageMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RelayMessageMenuItemContent {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StampImage")]
    pub m_stamp_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-relaymessagemenuitemcontent")]
#[::unity2::methods]
impl RelayMessageMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, data: crate::app::relaystampdata::RelayStampData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaymessagemenuitemcontent")]
impl RelayMessageMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMessageMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMessageMenuItemContentMethods>::ctor(this);
        this
    }
}
