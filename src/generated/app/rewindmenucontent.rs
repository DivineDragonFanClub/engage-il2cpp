
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenucontent/RewindMenuContent_Phase.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuContent.Phase")]
#[parent(crate::system::object::Object)]
pub struct RewindMenuContent_Phase {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TurnName")]
    pub m_turn_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Turn")]
    pub m_turn: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RestName")]
    pub m_rest_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Rest")]
    pub m_rest: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-rewindmenucontent")]
#[::unity2::methods]
impl RewindMenuContent_Phase {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetTurn", args = 1)]
    pub fn set_turn(self, turn: i32) -> ();

    #[method(name = "SetRestUnitNum", args = 1)]
    pub fn set_rest_unit_num(self, rest_unit_num: i32) -> ();
}

#[cfg(feature = "app-rewindmenucontent")]
impl RewindMenuContent_Phase {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuContent_Phase),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuContent_PhaseMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenucontent/RewindMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RewindMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ShowRowMax")]
    pub show_row_max: i32,
    #[rename(name = "m_PlayerPhaseObj")]
    pub m_player_phase_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EnemyPhaseObj")]
    pub m_enemy_phase_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AllyPhaseObj")]
    pub m_ally_phase_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Enemy2PhaseObj")]
    pub m_enemy2_phase_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ScrollArrowUpObj")]
    pub m_scroll_arrow_up_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ScrollArrowDownObj")]
    pub m_scroll_arrow_down_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RestRewindTimesObj")]
    pub m_rest_rewind_times_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KeyHelpObj")]
    pub m_key_help_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ForcePlayerColor")]
    pub m_force_player_color: crate::unity_engine::color::Color,
    #[rename(name = "m_ForceEnemyColor")]
    pub m_force_enemy_color: crate::unity_engine::color::Color,
    #[rename(name = "m_ForceAllyColor")]
    pub m_force_ally_color: crate::unity_engine::color::Color,
    #[rename(name = "m_ForceEnemy2Color")]
    pub m_force_enemy2_color: crate::unity_engine::color::Color,
    #[rename(name = "m_RootAnimator")]
    pub m_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PlayerPhase")]
    pub m_player_phase: crate::app::rewindmenucontent::RewindMenuContent_Phase,
    #[rename(name = "m_EnemyPhase")]
    pub m_enemy_phase: crate::app::rewindmenucontent::RewindMenuContent_Phase,
    #[rename(name = "m_AllyPhase")]
    pub m_ally_phase: crate::app::rewindmenucontent::RewindMenuContent_Phase,
    #[rename(name = "m_Enemy2Phase")]
    pub m_enemy2_phase: crate::app::rewindmenucontent::RewindMenuContent_Phase,
    #[rename(name = "m_ScrollArrowUp")]
    pub m_scroll_arrow_up: crate::app::rewindmenucontent::RewindMenuContent_ScrollArrow,
    #[rename(name = "m_ScrollArrowDown")]
    pub m_scroll_arrow_down: crate::app::rewindmenucontent::RewindMenuContent_ScrollArrow,
    #[rename(name = "m_RestRewindTimes")]
    pub m_rest_rewind_times: crate::app::rewindmenucontent::RewindMenuContent_RestRewindTimes,
    #[rename(name = "m_KeyHelp")]
    pub m_key_help: crate::app::rewindmenucontent::RewindMenuContent_KeyHelp,
}

#[cfg(feature = "app-rewindmenucontent")]
#[::unity2::methods]
impl RewindMenuContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "GetRewindMenuItemContent", args = 1)]
    pub fn get_rewind_menu_item_content(
        self,
        item_index: i32,
    ) -> crate::app::rewindmenuitemcontent::RewindMenuItemContent;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "BuildWH", args = 0)]
    pub fn build_wh(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CycleMenuItemContent", args = 2)]
    pub fn cycle_menu_item_content(self, is_forward: bool, cycle_count: i32) -> ();

    #[method(name = "GetLineHeightForScroll", args = 0)]
    pub fn get_line_height_for_scroll(self) -> f32;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "Suspend", args = 0)]
    pub fn suspend(self) -> ();

    #[method(name = "UnSuspend", args = 0)]
    pub fn un_suspend(self) -> ();

    #[method(name = "UpdateParts", args = 3)]
    pub fn update_parts(
        self,
        current_force_type: crate::app::force::Force_Type,
        turn: i32,
        rest_unit_num: i32,
    ) -> ();

    #[method(name = "IsMenuItemMoving", args = 0)]
    pub fn is_menu_item_moving(self) -> bool;

    #[method(name = "MoveUp", args = 1)]
    pub fn move_up(self, scroll_count: i32) -> ();

    #[method(name = "MoveDown", args = 1)]
    pub fn move_down(self, scroll_count: i32) -> ();

    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "CreateContent", args = 0)]
    pub fn create_content() -> crate::app::rewindmenucontent::RewindMenuContent;
}

#[cfg(feature = "app-rewindmenucontent")]
impl RewindMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenucontent/RewindMenuContent_RestRewindTimes.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuContent.RestRewindTimes")]
#[parent(crate::system::object::Object)]
pub struct RewindMenuContent_RestRewindTimes {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Times")]
    pub m_times: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-rewindmenucontent")]
#[::unity2::methods]
impl RewindMenuContent_RestRewindTimes {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetTimes", args = 1)]
    pub fn set_times(self, times: i32) -> ();
}

#[cfg(feature = "app-rewindmenucontent")]
impl RewindMenuContent_RestRewindTimes {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuContent_RestRewindTimes),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuContent_RestRewindTimesMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenucontent/RewindMenuContent_KeyHelp.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuContent.KeyHelp")]
#[parent(crate::system::object::Object)]
pub struct RewindMenuContent_KeyHelp {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-rewindmenucontent")]
#[::unity2::methods]
impl RewindMenuContent_KeyHelp {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Set", args = 0)]
    pub fn set(self) -> ();
}

#[cfg(feature = "app-rewindmenucontent")]
impl RewindMenuContent_KeyHelp {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuContent_KeyHelp),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuContent_KeyHelpMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenucontent/RewindMenuContent_ScrollArrow.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuContent.ScrollArrow")]
#[parent(crate::system::object::Object)]
pub struct RewindMenuContent_ScrollArrow {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-rewindmenucontent")]
#[::unity2::methods]
impl RewindMenuContent_ScrollArrow {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-rewindmenucontent")]
impl RewindMenuContent_ScrollArrow {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuContent_ScrollArrow),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuContent_ScrollArrowMethods>::ctor(this, root_object);
        this
    }
}
