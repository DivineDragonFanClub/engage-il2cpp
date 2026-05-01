
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponroot/RefineGodWeaponRoot_RefineStatusEfficacy.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponRoot.RefineStatusEfficacy")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponRoot_RefineStatusEfficacy {
    #[rename(name = "m_CaptionText")]
    pub m_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BeforeParentObject")]
    pub m_before_parent_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BeforeValueImage")]
    pub m_before_value_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_BeforeNothingText")]
    pub m_before_nothing_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AfterParentObject")]
    pub m_after_parent_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AfterValueImage")]
    pub m_after_value_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AfterNothingText")]
    pub m_after_nothing_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refinegodweaponroot")]
#[::unity2::methods]
impl RefineGodWeaponRoot_RefineStatusEfficacy {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refinegodweaponroot")]
impl RefineGodWeaponRoot_RefineStatusEfficacy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponRoot_RefineStatusEfficacy),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponRoot_RefineStatusEfficacyMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponroot/RefineGodWeaponRoot_RefineStatus.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponRoot.RefineStatus")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponRoot_RefineStatus {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    #[rename(name = "m_CaptionText")]
    pub m_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LvTitleText")]
    pub m_lv_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LvValueText")]
    pub m_lv_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BeforeParentObject")]
    pub m_before_parent_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BeforePlusText")]
    pub m_before_plus_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BeforeValueText")]
    pub m_before_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AfterParentObject")]
    pub m_after_parent_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AfterPlusText")]
    pub m_after_plus_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AfterValueText")]
    pub m_after_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refinegodweaponroot")]
#[::unity2::methods]
impl RefineGodWeaponRoot_RefineStatus {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refinegodweaponroot")]
impl RefineGodWeaponRoot_RefineStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponRoot_RefineStatus),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponRoot_RefineStatusMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponroot/RefineGodWeaponRoot_WeaponInfoLabel.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponRoot.WeaponInfoLabel")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponRoot_WeaponInfoLabel {
    #[rename(name = "m_KindFrameObject")]
    pub m_kind_frame_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponNameText")]
    pub m_weapon_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CapacityCaptionText")]
    pub m_capacity_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CapacityValueText")]
    pub m_capacity_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CapacityMaxText")]
    pub m_capacity_max_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refinegodweaponroot")]
#[::unity2::methods]
impl RefineGodWeaponRoot_WeaponInfoLabel {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refinegodweaponroot")]
impl RefineGodWeaponRoot_WeaponInfoLabel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponRoot_WeaponInfoLabel),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponRoot_WeaponInfoLabelMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponroot/RefineGodWeaponRoot.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineGodWeaponRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RefineGodWeaponSelectMenuContent")]
    pub m_refine_god_weapon_select_menu_content:
        crate::app::refinegodweaponselectmenucontent::RefineGodWeaponSelectMenuContent,
    #[rename(name = "m_RefineGodWeaponParamMenuContent")]
    pub m_refine_god_weapon_param_menu_content:
        crate::app::refinegodweaponparammenucontent::RefineGodWeaponParamMenuContent,
    #[rename(name = "m_ItemHelpObject")]
    pub m_item_help_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemMenuDetailSetter")]
    pub m_item_menu_detail_setter: crate::app::itemmenudetailsetter::ItemMenuDetailSetter,
    #[rename(name = "m_WeaponImage")]
    pub m_weapon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_WeaponInfoLabel")]
    pub m_weapon_info_label: crate::app::refinegodweaponroot::RefineGodWeaponRoot_WeaponInfoLabel,
    #[rename(name = "m_RefineStatusTitleText")]
    pub m_refine_status_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RefineStatus")]
    pub m_refine_status:
        ::unity2::Array<crate::app::refinegodweaponroot::RefineGodWeaponRoot_RefineStatus>,
    #[rename(name = "m_RefineStatusEfficacyParent")]
    pub m_refine_status_efficacy_parent: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RefineStatusEfficacy")]
    pub m_refine_status_efficacy:
        ::unity2::Array<crate::app::refinegodweaponroot::RefineGodWeaponRoot_RefineStatusEfficacy>,
    #[rename(name = "m_WeaponRotWaitTime")]
    pub m_weapon_rot_wait_time: f32,
    #[rename(name = "m_WeaponRotSpeedAuto")]
    pub m_weapon_rot_speed_auto: f32,
    #[rename(name = "m_WeaponRotSpeedMax")]
    pub m_weapon_rot_speed_max: f32,
    #[rename(name = "m_WeaponRotStickSense")]
    pub m_weapon_rot_stick_sense: f32,
    #[rename(name = "m_WeaponRotWaitTimeCount")]
    pub m_weapon_rot_wait_time_count: f32,
    #[static_field]
    #[rename(name = "m_RefineStatusEfficacyCaptionMid")]
    pub m_refine_status_efficacy_caption_mid: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_WeaponModelRenderer")]
    pub m_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
    #[rename(name = "m_UnitItemBase")]
    pub m_unit_item_base: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-refinegodweaponroot")]
#[::unity2::methods]
impl RefineGodWeaponRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateRoot", args = 1)]
    pub fn create_root(
        weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
    ) -> crate::app::refinegodweaponroot::RefineGodWeaponRoot;

    #[method(name = "Create", args = 1)]
    pub fn create(
        self,
        weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
    ) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "GetRefineGodWeaponSelectMenuContent", args = 0)]
    pub fn get_refine_god_weapon_select_menu_content(
        self,
    ) -> crate::app::refinegodweaponselectmenucontent::RefineGodWeaponSelectMenuContent;

    #[method(name = "GetRefineGodWeaponParamMenuContent", args = 0)]
    pub fn get_refine_god_weapon_param_menu_content(
        self,
    ) -> crate::app::refinegodweaponparammenucontent::RefineGodWeaponParamMenuContent;

    #[method(name = "OnSelect", args = 2)]
    pub fn on_select(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "OnSelectForParamMenu", args = 4)]
    pub fn on_select_for_param_menu(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_or_reset: bool,
    ) -> ();

    #[method(name = "SetNaviHelp", args = 1)]
    pub fn set_navi_help(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateItemDetail", args = 2)]
    pub fn update_item_detail(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "UpdateItemDetail", args = 4)]
    pub fn update_item_detail_2(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        refine_data_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_or_reset: bool,
    ) -> ();

    #[method(name = "UpdateWeaponModel", args = 2)]
    pub fn update_weapon_model(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "UpdateWeaponLabel", args = 2)]
    pub fn update_weapon_label(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "UpdateRefineStatus", args = 5)]
    pub fn update_refine_status(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        show_after: bool,
        refine_or_reset: bool,
    ) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refinegodweaponroot")]
impl RefineGodWeaponRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponRootMethods>::ctor(this);
        this
    }
}
