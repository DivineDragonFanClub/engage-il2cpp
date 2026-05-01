
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/damageinfo/DamageInfo_InfoComparer.md")))]
#[::unity2::class(namespace = "App", name = "DamageInfo.InfoComparer")]
#[parent(crate::system::object::Object)]
pub struct DamageInfo_InfoComparer {}

#[cfg(feature = "app-damageinfo")]
#[::unity2::methods]
impl DamageInfo_InfoComparer {
    #[method(name = "Compare", args = 2)]
    pub fn compare(
        self,
        a: crate::app::damageinfo::DamageInfo_Info,
        b: crate::app::damageinfo::DamageInfo_Info,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-damageinfo")]
impl DamageInfo_InfoComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DamageInfo_InfoComparer),
                ::core::stringify!(new),
            )
        });
        <Self as IDamageInfo_InfoComparerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/damageinfo/DamageInfo_InfoWindow.md")))]
#[::unity2::class(namespace = "App", name = "DamageInfo.InfoWindow")]
#[parent(crate::system::object::Object)]
pub struct DamageInfo_InfoWindow {
    #[rename(name = "m_IsShow")]
    pub m_is_show: bool,
    #[rename(name = "m_Info")]
    pub m_info: crate::app::damageinfo::DamageInfo_Info,
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WindowImage")]
    pub m_window_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_SkillIconImage")]
    pub m_skill_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_UnitNameText")]
    pub m_unit_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillNameText")]
    pub m_skill_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DamageText")]
    pub m_damage_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-damageinfo")]
#[::unity2::methods]
impl DamageInfo_InfoWindow {
    #[method(name = "IsShow", args = 0)]
    pub fn is_show(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "SetInfo", args = 1)]
    pub fn set_info(self, info: crate::app::damageinfo::DamageInfo_Info) -> ();
}

#[cfg(feature = "app-damageinfo")]
impl DamageInfo_InfoWindow {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DamageInfo_InfoWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IDamageInfo_InfoWindowMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/damageinfo/DamageInfo_Info.md")))]
#[::unity2::class(namespace = "App", name = "DamageInfo.Info")]
#[parent(crate::system::object::Object)]
pub struct DamageInfo_Info {}

#[cfg(feature = "app-damageinfo")]
#[::unity2::methods]
impl DamageInfo_Info {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Skill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_Skill", args = 1)]
    pub fn set_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_Damage", args = 0)]
    pub fn get_damage(self) -> i32;

    #[method(name = "set_Damage", args = 1)]
    pub fn set_damage(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-damageinfo")]
impl DamageInfo_Info {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DamageInfo_Info),
                ::core::stringify!(new),
            )
        });
        <Self as IDamageInfo_InfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/damageinfo/DamageInfo.md")))]
#[::unity2::class(namespace = "App", name = "DamageInfo")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: damageinfo :: DamageInfo >)]
pub struct DamageInfo {
    #[static_field]
    #[rename(name = "InfoWindowMax")]
    pub info_window_max: i32,
    #[rename(name = "m_IsShow")]
    pub m_is_show: bool,
    #[rename(name = "m_LayoutPrefab")]
    pub m_layout_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_InfoList")]
    pub m_info_list: crate::system::collections::generic::list_1::List_1<
        crate::app::damageinfo::DamageInfo_Info,
    >,
    #[rename(name = "m_InfoWindowArray")]
    pub m_info_window_array: ::unity2::Array<crate::app::damageinfo::DamageInfo_InfoWindow>,
    #[rename(name = "m_BattleInfo")]
    pub m_battle_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_BattleCalculator")]
    pub m_battle_calculator: crate::app::battlecalculator::BattleCalculator,
    #[rename(name = "m_PreDefenceUnit")]
    pub m_pre_defence_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-damageinfo")]
#[::unity2::methods]
impl DamageInfo {
    #[method(name = "get_GlobalAssetPath", args = 0)]
    pub fn get_global_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadLayoutPrefab", args = 0)]
    pub fn load_layout_prefab(self) -> ();

    #[method(name = "IsLoadingLayoutPrefab", args = 0)]
    pub fn is_loading_layout_prefab(self) -> bool;

    #[method(name = "IsShow", args = 0)]
    pub fn is_show(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CalcInfoList", args = 1)]
    pub fn calc_info_list(self, defence_unit: crate::app::unit::Unit) -> ();

    #[method(name = "CalcInfoList", args = 0)]
    pub fn calc_info_list_2(self) -> ();

    #[method(name = "CalcDamage", args = 3)]
    pub fn calc_damage(
        self,
        offence_unit: crate::app::unit::Unit,
        defence_unit: crate::app::unit::Unit,
        engage_skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "GetProcDesc", args = 0)]
    pub fn get_proc_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateInfo", args = 0)]
    pub fn update_info() -> ();

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(is_visible: bool) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-damageinfo")]
impl DamageInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DamageInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IDamageInfoMethods>::ctor(this);
        this
    }
}
