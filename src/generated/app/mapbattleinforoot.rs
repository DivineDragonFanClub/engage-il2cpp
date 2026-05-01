
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinforoot/MapBattleInfoRoot_StatusShowType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapBattleInfoRoot_StatusShowType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapBattleInfoRoot_StatusShowType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapBattleInfoRoot.StatusShowType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapBattleInfoRoot_StatusShowType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapBattleInfoRoot_StatusShowType {
    pub fn hide() -> Self {
        Self { value: 0 }
    }

    pub fn full() -> Self {
        Self { value: 1 }
    }

    pub fn hp_only() -> Self {
        Self { value: 2 }
    }

    pub fn hit_only() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinforoot/MapBattleInfoRoot.md")))]
#[::unity2::class(namespace = "App", name = "MapBattleInfoRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapBattleInfoRoot {
    #[rename(name = "m_FrameRoot")]
    pub m_frame_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CommandRoot")]
    pub m_command_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CommandSubRoot")]
    pub m_command_sub_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CommandText")]
    pub m_command_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CommandSubText")]
    pub m_command_sub_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_InfoLeft")]
    pub m_info_left: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_InfoRight")]
    pub m_info_right: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mapbattleinforoot")]
#[::unity2::methods]
impl MapBattleInfoRoot {
    #[method(name = "IsDestroy", args = 1)]
    pub fn is_destroy(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsDance", args = 1)]
    pub fn is_dance(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsFireCannon", args = 1)]
    pub fn is_fire_cannon(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsEngageRodRangeAgain", args = 2)]
    pub fn is_engage_rod_range_again(
        info: crate::app::battleinfo::BattleInfo,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "IsEngageRodRangeHeal", args = 2)]
    pub fn is_engage_rod_range_heal(
        info: crate::app::battleinfo::BattleInfo,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "IsSupoortCommandSkill", args = 0)]
    pub fn is_supoort_command_skill() -> bool;

    #[method(name = "IsRodInterference", args = 2)]
    pub fn is_rod_interference(
        side_type: crate::app::battleside::BattleSide_Type,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "IsRodRangeHeal", args = 1)]
    pub fn is_rod_range_heal(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsRodCreation", args = 1)]
    pub fn is_rod_creation(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsRodRewarp", args = 1)]
    pub fn is_rod_rewarp(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsNoShowParamRod", args = 1)]
    pub fn is_no_show_param_rod(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsRecoveryHP", args = 2)]
    pub fn is_recovery_hp(
        side_type: crate::app::battleside::BattleSide_Type,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> bool;

    #[method(name = "IsSelfRecoveryHP", args = 3)]
    pub fn is_self_recovery_hp(
        side_type: crate::app::battleside::BattleSide_Type,
        info: crate::app::battleinfo::BattleInfo,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> bool;

    #[method(name = "IsSelfTarget", args = 1)]
    pub fn is_self_target(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsNotExistTarget", args = 1)]
    pub fn is_not_exist_target(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsShowParam", args = 2)]
    pub fn is_show_param(
        side: crate::app::battleinfoside::BattleInfoSide,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> bool;

    #[method(name = "IsEngageRodBless", args = 2)]
    pub fn is_engage_rod_bless(
        info: crate::app::battleinfo::BattleInfo,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "IsEngageRodBlessRest", args = 2)]
    pub fn is_engage_rod_bless_rest(
        info: crate::app::battleinfo::BattleInfo,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "IsRodRest", args = 1)]
    pub fn is_rod_rest(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsEnchantHeal", args = 1)]
    pub fn is_enchant_heal(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsEnchantRest", args = 1)]
    pub fn is_enchant_rest(item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "Setup", args = 4)]
    pub fn setup(
        self,
        mind_type: crate::app::mapmind::MapMind_Type,
        skill: crate::app::skilldata::SkillData,
        info: crate::app::battleinfo::BattleInfo,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> bool;

    #[method(name = "SetCommandText", args = 1)]
    pub fn set_command_text(self, mind_type: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "SetCommandText", args = 1)]
    pub fn set_command_text_2(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "SetCommandText", args = 1)]
    pub fn set_command_text_3(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "SetEngageCommandVisible", args = 3)]
    pub fn set_engage_command_visible(
        self,
        mind_type: crate::app::mapmind::MapMind_Type,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbattleinforoot")]
impl MapBattleInfoRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBattleInfoRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBattleInfoRootMethods>::ctor(this);
        this
    }
}
