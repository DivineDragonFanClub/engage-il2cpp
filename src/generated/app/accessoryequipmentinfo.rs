
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryequipmentinfo/AccessoryEquipmentInfo.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryEquipmentInfo")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct AccessoryEquipmentInfo {
    #[rename(name = "m_ContentObject")]
    pub m_content_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorObject")]
    pub m_cursor_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MenuItemList")]
    pub m_menu_item_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
}

#[cfg(feature = "app-accessoryequipmentinfo")]
#[::unity2::methods]
impl AccessoryEquipmentInfo {
    #[method(name = "Create", args = 0)]
    pub fn create() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, default_unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetItemContentMax", args = 0)]
    pub fn get_item_content_max(self) -> i32;

    #[method(name = "ShowCursor", args = 0)]
    pub fn show_cursor(self) -> ();

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor_2(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor_3(self, kind: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = "HideCursor", args = 0)]
    pub fn hide_cursor(self) -> ();
}

#[cfg(feature = "app-accessoryequipmentinfo")]
impl AccessoryEquipmentInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryEquipmentInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryEquipmentInfoMethods>::ctor(this);
        this
    }
}
