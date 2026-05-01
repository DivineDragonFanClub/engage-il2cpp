
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopunitselectroot/AccessoryShopUnitSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopUnitSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct AccessoryShopUnitSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_ShopUnitSelectMenuObject")]
    pub m_shop_unit_select_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AccessoryEquipmentInfoObject")]
    pub m_accessory_equipment_info_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AccessoryEquipmentInfoWindow")]
    pub m_accessory_equipment_info_window:
        crate::app::accessoryequipmentinfo::AccessoryEquipmentInfo,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-accessoryshopunitselectroot")]
#[::unity2::methods]
impl AccessoryShopUnitSelectRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        default_unit: crate::app::unit::Unit,
        default_scroll_index: i32,
        decide_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
    ) -> crate::app::accessoryshopunitselectroot::AccessoryShopUnitSelectRoot;

    #[method(name = "Create", args = 4)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        default_unit: crate::app::unit::Unit,
        default_scroll_index: i32,
        decide_event_handler: crate::app::shopunitselectmenu::ShopUnitSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(
        root: crate::app::accessoryshopunitselectroot::AccessoryShopUnitSelectRoot,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnDecideMenuItem", args = 3)]
    pub fn on_decide_menu_item(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        unit: crate::app::unit::Unit,
        scroll_index: i32,
    ) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();
}

#[cfg(feature = "app-accessoryshopunitselectroot")]
impl AccessoryShopUnitSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopUnitSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopUnitSelectRootMethods>::ctor(this);
        this
    }
}
