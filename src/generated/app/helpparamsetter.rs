
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/helpparamsetter/HelpParamSetter.md")))]
#[::unity2::class(namespace = "App", name = "HelpParamSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HelpParamSetter {
    #[rename(name = "m_CursorObj")]
    pub m_cursor_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WindowObj")]
    pub m_window_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TitleRoot")]
    pub m_title_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponRoot")]
    pub m_weapon_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MessageRoot")]
    pub m_message_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemIcon")]
    pub m_item_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillIcon")]
    pub m_skill_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ContentName")]
    pub m_content_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Endurance")]
    pub m_endurance: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TitleAtk")]
    pub m_title_atk: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ValueAtk")]
    pub m_value_atk: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconAtk")]
    pub m_icon_atk: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ValueHit")]
    pub m_value_hit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconHit")]
    pub m_icon_hit: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ValueCrit")]
    pub m_value_crit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconCrit")]
    pub m_icon_crit: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ValueSpd")]
    pub m_value_spd: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconSpd")]
    pub m_icon_spd: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ValueAvo")]
    pub m_value_avo: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconAvo")]
    pub m_icon_avo: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ValueCritAvo")]
    pub m_value_crit_avo: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconCritAvo")]
    pub m_icon_crit_avo: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_TitleRange")]
    pub m_title_range: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ValueRange")]
    pub m_value_range: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EfficacyNothing")]
    pub m_efficacy_nothing: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EfficacyIcons")]
    pub m_efficacy_icons: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_TitleWeaponLevel")]
    pub m_title_weapon_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ValueWeaponLevel")]
    pub m_value_weapon_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IconWeaponLevel")]
    pub m_icon_weapon_level: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ContentsText")]
    pub m_contents_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ContentsEngWep")]
    pub m_contents_eng_wep: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ContentsEnchant")]
    pub m_contents_enchant: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ContentsSubText")]
    pub m_contents_sub_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PrevPosition")]
    pub m_prev_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_PrevSize")]
    pub m_prev_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_NextPosition")]
    pub m_next_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_NextSize")]
    pub m_next_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_CursorMoveTimer")]
    pub m_cursor_move_timer: f32,
    #[rename(name = "m_DefaultWindowPos")]
    pub m_default_window_pos: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_PrevWindowPos")]
    pub m_prev_window_pos: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_NextWindowPos")]
    pub m_next_window_pos: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_PrevWindowPivot")]
    pub m_prev_window_pivot: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_NextWindowPivot")]
    pub m_next_window_pivot: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_PrevWindowAnchor")]
    pub m_prev_window_anchor: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_NextWindowAnchor")]
    pub m_next_window_anchor: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_CursorMoveTime")]
    pub m_cursor_move_time: f32,
    #[rename(name = "m_BattleInfo")]
    pub m_battle_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_TmpBattleInfo")]
    pub m_tmp_battle_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_TmpCalcUnit")]
    pub m_tmp_calc_unit: crate::app::unit::Unit,
    #[rename(name = "m_enhancedValue")]
    pub m_enhanced_value: ::unity2::Array<i32>,
    #[rename(name = "m_enhancedTitle")]
    pub m_enhanced_title: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_isGodChange")]
    pub m_is_god_change: bool,
    #[rename(name = "m_isRingChange")]
    pub m_is_ring_change: bool,
    #[rename(name = "m_ringSelectGod")]
    pub m_ring_select_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_ringSelectCommon")]
    pub m_ring_select_common: crate::app::unitring::UnitRing,
    #[static_field]
    #[rename(name = "WeaponKindMidTable")]
    pub weapon_kind_mid_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "WeaponKindMidBullet")]
    pub weapon_kind_mid_bullet: ::unity2::Il2CppString,
}

#[cfg(feature = "app-helpparamsetter")]
#[::unity2::methods]
impl HelpParamSetter {
    #[method(name = "SetRingSelectGod", args = 2)]
    pub fn set_ring_select_god(self, is_temp_god: bool, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "SetRingSelectCommon", args = 2)]
    pub fn set_ring_select_common(
        self,
        is_temp_ring: bool,
        ring: crate::app::unitring::UnitRing,
    ) -> ();

    #[method(name = "MoveImmediate", args = 0)]
    pub fn move_immediate(self) -> ();

    #[method(name = "UpdateLangage", args = 0)]
    pub fn update_langage(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetItemFixedText", args = 0)]
    pub fn set_item_fixed_text(self) -> ();

    #[method(name = "TrySetUpDownIcon", args = 4)]
    pub fn try_set_up_down_icon(
        self,
        icon: crate::unity_engine::ui::image::Image,
        text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        before: i32,
        now: i32,
    ) -> ();

    #[method(name = "MoveCursor", args = 1)]
    pub fn move_cursor(self, frame: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "UpdateCursorPosition", args = 0)]
    pub fn update_cursor_position(self) -> ();

    #[method(name = "CurveDecel", args = 5)]
    pub fn curve_decel(
        self,
        prev: crate::unity_engine::vector2::Vector2,
        next: crate::unity_engine::vector2::Vector2,
        now: f32,
        term: f32,
        num: i32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetHelpText", args = 1)]
    pub fn get_help_text(self, help: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "SetPerson", args = 2)]
    pub fn set_person(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "IsVeyreForNetwork", args = 1)]
    pub fn is_veyre_for_network(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetJob", args = 2)]
    pub fn set_job(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        job: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "SetBattleStyle", args = 2)]
    pub fn set_battle_style(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        style: crate::app::battlestyle::BattleStyle_Types,
    ) -> ();

    #[method(name = "SetEfficacy", args = 2)]
    pub fn set_efficacy(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "SetItemData", args = 8)]
    pub fn set_item_data(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        data: crate::app::itemdata::ItemData,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::unitring::UnitRing,
        endurance: i32,
        item: crate::app::unititem::UnitItem,
        is_use_enchant: bool,
    ) -> ();

    #[method(name = "SetRangeText", args = 2)]
    pub fn set_range_text(self, r_i: i32, r_o: i32) -> ();

    #[method(name = "SetUnitItem", args = 6)]
    pub fn set_unit_item(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        item: crate::app::unititem::UnitItem,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::unitring::UnitRing,
        is_use_enchant: bool,
    ) -> ();

    #[method(name = "SetSkill", args = 4)]
    pub fn set_skill(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        skill: crate::app::skilldata::SkillData,
        b_engage: bool,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "SetCapability", args = 3)]
    pub fn set_capability(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> ();

    #[method(name = "SetBattleInfo", args = 4)]
    pub fn set_battle_info(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        r#type: crate::app::unitinfoparammanager::UnitInfoParamManager_ValueType,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddEnhancedString", args = 3)]
    pub fn add_enhanced_string(
        self,
        info_text: ::unity2::Il2CppString,
        title: ::unity2::Il2CppString,
        value: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SetGod", args = 3)]
    pub fn set_god(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        temp_god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "SetGod", args = 2)]
    pub fn set_god_2(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "SetWeaponLevel", args = 3)]
    pub fn set_weapon_level(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        job_data: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "SetEnchantment", args = 2)]
    pub fn set_enchantment(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetFixedText", args = 2)]
    pub fn set_fixed_text(
        self,
        frame: crate::unity_engine::gameobject::GameObject,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-helpparamsetter")]
impl HelpParamSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HelpParamSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IHelpParamSetterMethods>::ctor(this);
        this
    }
}
