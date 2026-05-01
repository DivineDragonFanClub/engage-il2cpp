
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenuitemcontent/RefineShopEngraveGodMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RefineShopEngraveGodMenuItemContent {
    #[rename(name = "m_GodSymbolImage")]
    pub m_god_symbol_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_GodWordText")]
    pub m_god_word_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodUnitIcon")]
    pub m_god_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_BondsIconObject")]
    pub m_bonds_icon_object: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_BondsValueText")]
    pub m_bonds_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineshopengravegodmenuitemcontent")]
#[::unity2::methods]
impl RefineShopEngraveGodMenuItemContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodmenuitemcontent")]
impl RefineShopEngraveGodMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenuItemContentMethods>::ctor(this);
        this
    }
}
