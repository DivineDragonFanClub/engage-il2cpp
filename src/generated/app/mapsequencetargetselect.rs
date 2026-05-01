
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencetargetselect/MapSequenceTargetSelect_InfoType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceTargetSelect_InfoType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceTargetSelect_InfoType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceTargetSelect.InfoType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceTargetSelect_InfoType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceTargetSelect_InfoType {
    pub fn common() -> Self {
        Self { value: 0 }
    }

    pub fn battle() -> Self {
        Self { value: 1 }
    }

    pub fn rod() -> Self {
        Self { value: 2 }
    }

    pub fn rod_only() -> Self {
        Self { value: 3 }
    }

    pub fn item() -> Self {
        Self { value: 4 }
    }

    pub fn destroy() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencetargetselect/MapSequenceTargetSelect.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceTargetSelect")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapsequencetargetselect :: MapSequenceTargetSelect >)]
pub struct MapSequenceTargetSelect {
    #[rename(name = "m_TargetData")]
    pub m_target_data: crate::app::maptarget::MapTarget_Data,
    #[rename(name = "m_ItemIndex")]
    pub m_item_index: i32,
    #[rename(name = "m_BattleInfo")]
    pub m_battle_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_BattleCalc")]
    pub m_battle_calc: crate::app::battlecalculator::BattleCalculator,
    #[rename(name = "m_EngageLinkInfo")]
    pub m_engage_link_info: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MaskSkill")]
    pub m_mask_skill: crate::app::skillarray::SkillArray,
}

#[cfg(feature = "app-mapsequencetargetselect")]
#[::unity2::methods]
impl MapSequenceTargetSelect {
    #[method(name = "get_GlobalAssetPath", args = 0)]
    pub fn get_global_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "WaitForLoad", args = 0)]
    pub fn wait_for_load(self) -> ();

    #[method(name = "IsCursorSelectedCommand", args = 0)]
    pub fn is_cursor_selected_command(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "SyncHpGaugeFlash", args = 1)]
    pub fn sync_hp_gauge_flash(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "SetMapCursorPositon", args = 0)]
    pub fn set_map_cursor_positon(self) -> ();

    #[method(name = "SetHpForecast", args = 0)]
    pub fn set_hp_forecast(self) -> ();

    #[method(name = "TryChangeEngage", args = 0)]
    pub fn try_change_engage(self) -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "TryChangeWeapon", args = 0)]
    pub fn try_change_weapon(self) -> bool;

    #[method(name = "CharaOnlyOn", args = 2)]
    pub fn chara_only_on(
        self,
        target_unit: crate::app::unit::Unit,
        target_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "WindowOut", args = 0)]
    pub fn window_out(self) -> ();

    #[method(name = "PostWindowOut", args = 0)]
    pub fn post_window_out(self) -> ();

    #[method(name = "SetMapMind", args = 0)]
    pub fn set_map_mind(self) -> ();

    #[method(name = "SetMapMind", args = 3)]
    pub fn set_map_mind_2(
        self,
        mind: crate::app::mapmind::MapMind_Type,
        item_index: i32,
        target_index: i32,
    ) -> ();

    #[method(name = "SetMapMind", args = 2)]
    pub fn set_map_mind_3(self, target_x: i32, target_z: i32) -> ();

    #[method(name = "IsEquipped", args = 0)]
    pub fn is_equipped(self) -> bool;

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = "SetItemSelectedForDecide", args = 0)]
    pub fn set_item_selected_for_decide(self) -> ();

    #[method(name = "SetMapCursorDisplayForDecide", args = 0)]
    pub fn set_map_cursor_display_for_decide(self) -> ();

    #[method(name = "SetCircleAnimeForDecide", args = 0)]
    pub fn set_circle_anime_for_decide(self) -> ();

    #[method(name = "ItemEquip", args = 2)]
    pub fn item_equip(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "DecideNormal", args = 0)]
    pub fn decide_normal(self) -> ();

    #[method(name = "DecideDesignate", args = 0)]
    pub fn decide_designate(self) -> ();

    #[method(name = "GetBackToItemSelectMenu", args = 1)]
    pub fn get_back_to_item_select_menu(
        self,
        mind_type: crate::app::mapmind::MapMind_Type,
    ) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "SetItemSelectedForCancel", args = 0)]
    pub fn set_item_selected_for_cancel(self) -> ();

    #[method(name = "SetMapCursorPosForCancel", args = 0)]
    pub fn set_map_cursor_pos_for_cancel(self) -> ();

    #[method(name = "SetMapCursorDisplayForCancel", args = 0)]
    pub fn set_map_cursor_display_for_cancel(self) -> ();

    #[method(name = "SetCircleAnimeForCancel", args = 0)]
    pub fn set_circle_anime_for_cancel(self) -> ();

    #[method(name = "SetMapTerrainInfoForCancel", args = 0)]
    pub fn set_map_terrain_info_for_cancel(self) -> ();

    #[method(name = "TryBackUnitCommand", args = 0)]
    pub fn try_back_unit_command(self) -> ();

    #[method(name = "CancelNormal", args = 0)]
    pub fn cancel_normal(self) -> ();

    #[method(name = "CancelDesignate", args = 0)]
    pub fn cancel_designate(self) -> ();

    #[method(name = "CanSelectTarget", args = 0)]
    pub fn can_select_target(self) -> bool;

    #[method(name = "IsAutoEquip", args = 0)]
    pub fn is_auto_equip(self) -> bool;

    #[method(name = "SetupInitialTarget", args = 0)]
    pub fn setup_initial_target(self) -> ();

    #[method(name = "TrySetTarget", args = 2)]
    pub fn try_set_target(
        self,
        data: crate::app::maptarget::MapTarget_Data,
        item_index: i32,
    ) -> bool;

    #[method(name = "UpdateInfo", args = 2)]
    pub fn update_info(self, target: crate::app::unit::Unit, item_index: i32) -> ();

    #[method(name = "SetItemIndex", args = 1)]
    pub fn set_item_index(self, item_index: i32) -> ();

    #[method(name = "ChangeToPrevTarget", args = 0)]
    pub fn change_to_prev_target(self) -> ();

    #[method(name = "ChangeToNextTarget", args = 0)]
    pub fn change_to_next_target(self) -> ();

    #[method(name = "IsItemChangeEnable", args = 2)]
    pub fn is_item_change_enable(self, now_index: i32, next_index: i32) -> bool;

    #[method(name = "CheckItemChange", args = 0)]
    pub fn check_item_change(self) -> bool;

    #[method(name = "ChangeToPrevItem", args = 0)]
    pub fn change_to_prev_item(self) -> ();

    #[method(name = "ChangeToNextItem", args = 0)]
    pub fn change_to_next_item(self) -> ();

    #[method(name = "UpdateMapPanelActive", args = 0)]
    pub fn update_map_panel_active(self) -> ();

    #[method(name = "UpdateSelect", args = 0)]
    pub fn update_select(self) -> ();

    #[method(name = "UpdateInfo", args = 0)]
    pub fn update_info_2(self) -> ();

    #[method(name = "UpdateActor", args = 0)]
    pub fn update_actor(self) -> ();

    #[method(name = "GetInfoType", args = 0)]
    pub fn get_info_type(
        self,
    ) -> crate::app::mapsequencetargetselect::MapSequenceTargetSelect_InfoType;

    #[method(name = "IsFireCannon", args = 0)]
    pub fn is_fire_cannon(self) -> bool;

    #[method(name = "IsBattle", args = 0)]
    pub fn is_battle(self) -> bool;

    #[method(name = "GetSelectItem", args = 0)]
    pub fn get_select_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "IsWeaponSelect", args = 0)]
    pub fn is_weapon_select(self) -> bool;

    #[method(name = "GetBattleInfo", args = 0)]
    pub fn get_battle_info(self) -> crate::app::battleinfo::BattleInfo;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetTargetUnit", args = 0)]
    pub fn get_target_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetTargetUnitIndex", args = 0)]
    pub fn get_target_unit_index(self) -> i32;

    #[method(name = "GetTargetX", args = 0)]
    pub fn get_target_x(self) -> i32;

    #[method(name = "GetTargetZ", args = 0)]
    pub fn get_target_z(self) -> i32;

    #[method(name = "GetMaskSkill", args = 0)]
    pub fn get_mask_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mapsequencetargetselect")]
impl MapSequenceTargetSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceTargetSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceTargetSelectMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencetargetselect/MapSequenceTargetSelect_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceTargetSelect_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceTargetSelect_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceTargetSelect.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceTargetSelect_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceTargetSelect_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}
