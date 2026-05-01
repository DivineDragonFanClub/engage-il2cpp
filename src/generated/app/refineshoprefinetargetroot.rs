
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetroot/RefineShopRefineTargetRoot.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineTargetRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineShopRefineTargetRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RefineShopRefineTargetMenuObject")]
    pub m_refine_shop_refine_target_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemDetailWindowBase")]
    pub m_item_detail_window_base: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
    #[rename(name = "m_ItemDetailWindowTarget")]
    pub m_item_detail_window_target: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
    #[rename(name = "m_ArrowObject")]
    pub m_arrow_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::refineshoprefinetargetroot::RefineShopRefineTargetRoot_ReturnEventHandler,
    #[rename(name = "m_RefineShopRefineTargetMenu")]
    pub m_refine_shop_refine_target_menu:
        crate::app::refineshoprefinetargetmenu::RefineShopRefineTargetMenu,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_ItemIndex")]
    pub m_item_index: i32,
    #[rename(name = "m_BaseUnitItem")]
    pub m_base_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_RefineLevel")]
    pub m_refine_level: i32,
    #[rename(name = "m_EvolveIndex")]
    pub m_evolve_index: i32,
}

#[cfg(feature = "app-refineshoprefinetargetroot")]
#[::unity2::methods]
impl RefineShopRefineTargetRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        unit: crate::app::unit::Unit,
        item_index: i32,
        return_event_handler : crate :: app :: refineshoprefinetargetroot :: RefineShopRefineTargetRoot_ReturnEventHandler,
    ) -> crate::app::refineshoprefinetargetroot::RefineShopRefineTargetRoot;

    #[method(name = "Create", args = 5)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        unit: crate::app::unit::Unit,
        item_index: i32,
        return_event_handler : crate :: app :: refineshoprefinetargetroot :: RefineShopRefineTargetRoot_ReturnEventHandler,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(root: crate::app::refineshoprefinetargetroot::RefineShopRefineTargetRoot) -> ();

    #[method(name = "UpdateItemDetail", args = 3)]
    pub fn update_item_detail(
        self,
        item_detail_window: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
        unit_item_base: crate::app::unititem::UnitItem,
        revealed: bool,
    ) -> ();

    #[method(name = "UpdateItemDetail", args = 4)]
    pub fn update_item_detail_2(
        self,
        item_detail_window: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
        unit_item_base: crate::app::unititem::UnitItem,
        unit_item_target: crate::app::unititem::UnitItem,
        revealed: bool,
    ) -> ();

    #[method(name = "OnSelectMenuItem", args = 2)]
    pub fn on_select_menu_item(
        self,
        unit_item: crate::app::unititem::UnitItem,
        revealed: bool,
    ) -> ();

    #[method(name = "OnDecideMenuItemToRefine", args = 6)]
    pub fn on_decide_menu_item_to_refine(
        self,
        refine_level: i32,
        refined_unit_item: crate::app::unititem::UnitItem,
        iron_num: i32,
        steel_num: i32,
        silver_num: i32,
        price: i32,
    ) -> ();

    #[method(name = "OnRefine", args = 0)]
    pub fn on_refine(self) -> ();

    #[method(name = "OnDecideMenuItemToEvolve", args = 2)]
    pub fn on_decide_menu_item_to_evolve(
        self,
        evolved_unit_item: crate::app::unititem::UnitItem,
        evolve_index: i32,
    ) -> ();

    #[method(name = "OnEvolve", args = 0)]
    pub fn on_evolve(self) -> ();

    #[method(name = "OnRequestCloseMenu", args = 0)]
    pub fn on_request_close_menu(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshoprefinetargetroot")]
impl RefineShopRefineTargetRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetRootMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetroot/RefineShopRefineTargetRoot_ReturnEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopRefineTargetRoot.ReturnEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopRefineTargetRoot_ReturnEventHandler {}

#[cfg(feature = "app-refineshoprefinetargetroot")]
#[::unity2::methods]
impl RefineShopRefineTargetRoot_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem) -> ();
}

#[cfg(feature = "app-refineshoprefinetargetroot")]
impl RefineShopRefineTargetRoot_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetRoot_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetRoot_ReturnEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
