
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectmenuitemcontent/UnitSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct UnitSelectMenuItemContent {
    #[rename(name = "m_Setter")]
    pub m_setter: crate::app::unitmenuitemsetter::UnitMenuItemSetter,
    #[rename(name = "m_relayOthers")]
    pub m_relay_others: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_versusDefense")]
    pub m_versus_defense: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_subFrameType")]
    pub m_sub_frame_type: crate::app::unitmenuitemsetter::UnitMenuItemSetter_SubFrame,
    #[rename(name = "m_IsSortieBackupUnit")]
    pub m_is_sortie_backup_unit: bool,
}

#[cfg(feature = "app-unitselectmenuitemcontent")]
#[::unity2::methods]
impl UnitSelectMenuItemContent {
    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = "SetupObjects", args = 0)]
    pub fn setup_objects(self) -> ();

    #[method(name = "GetMenuItemUnit", args = 0)]
    pub fn get_menu_item_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "UpdateUnitStatus", args = 0)]
    pub fn update_unit_status(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectmenuitemcontent")]
impl UnitSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
