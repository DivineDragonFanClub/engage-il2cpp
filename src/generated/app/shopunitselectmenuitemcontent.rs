
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopunitselectmenuitemcontent/ShopUnitSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ShopUnitSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ShopUnitSelectMenuItemContent {
    #[rename(name = "m_Setter")]
    pub m_setter: crate::app::unitmenuitemsetter::UnitMenuItemSetter,
}

#[cfg(feature = "app-shopunitselectmenuitemcontent")]
#[::unity2::methods]
impl ShopUnitSelectMenuItemContent {
    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopunitselectmenuitemcontent")]
impl ShopUnitSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopUnitSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IShopUnitSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
