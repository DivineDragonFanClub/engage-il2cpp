
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectroot/MyRoomWakeupSelectRoot_CursorTop.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectRoot.CursorTop")]
#[parent(crate::system::object::Object)]
pub struct MyRoomWakeupSelectRoot_CursorTop {
    #[static_field]
    #[rename(name = "TYPE_MAX")]
    pub type_max: i32,
    #[rename(name = "ResetRect")]
    pub reset_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "RankC_PosX")]
    pub rank_c_pos_x: f32,
    #[rename(name = "RankB_PosX")]
    pub rank_b_pos_x: f32,
    #[rename(name = "RankA_PosX")]
    pub rank_a_pos_x: f32,
    #[rename(name = "RankS_PosX")]
    pub rank_s_pos_x: f32,
    #[rename(name = "Upper_PosY")]
    pub upper_pos_y: f32,
    #[rename(name = "Lower_PosY")]
    pub lower_pos_y: f32,
    #[rename(name = "MoveFrame")]
    pub move_frame: f32,
    #[rename(name = "m_cursorTop")]
    pub m_cursor_top: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_selectIndex")]
    pub m_select_index: i32,
    #[rename(name = "m_prevIndex")]
    pub m_prev_index: i32,
    #[rename(name = "m_moveTick")]
    pub m_move_tick: f32,
}

#[cfg(feature = "app-myroomwakeupselectroot")]
#[::unity2::methods]
impl MyRoomWakeupSelectRoot_CursorTop {
    #[method(name = "IsOpen", args = 1)]
    pub fn is_open(self, index: i32) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, transform: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "Reset", args = 3)]
    pub fn reset(
        self,
        transform: crate::unity_engine::recttransform::RectTransform,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> ();

    #[method(name = "SetSelectIndex", args = 1)]
    pub fn set_select_index(self, select_index: i32) -> ();

    #[method(name = "GetSelectIndex", args = 0)]
    pub fn get_select_index(self) -> i32;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "GetCorrectedYCoord", args = 0)]
    pub fn get_corrected_y_coord(self) -> f32;

    #[method(name = "GetPositionX", args = 1)]
    pub fn get_position_x(self, select_index: i32) -> f32;

    #[method(name = "GetPositionY", args = 1)]
    pub fn get_position_y(self, select_index: i32) -> f32;
}

#[cfg(feature = "app-myroomwakeupselectroot")]
impl MyRoomWakeupSelectRoot_CursorTop {
    pub fn new(transform: crate::unity_engine::recttransform::RectTransform) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectRoot_CursorTop),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectRoot_CursorTopMethods>::ctor(this, transform);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectroot/MyRoomWakeupSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MyRoomWakeupSelectRoot {
    #[rename(name = "m_RelianceSelect")]
    pub m_reliance_select: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BackCursor")]
    pub m_back_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorTop")]
    pub m_cursor_top: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CharaImageRight")]
    pub m_chara_image_right: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
}

#[cfg(feature = "app-myroomwakeupselectroot")]
#[::unity2::methods]
impl MyRoomWakeupSelectRoot {
    #[method(name = "get_Cursor", args = 0)]
    pub fn get_cursor(self)
        -> crate::app::myroomwakeupselectroot::MyRoomWakeupSelectRoot_CursorTop;

    #[method(name = "set_Cursor", args = 1)]
    pub fn set_cursor(
        self,
        value: crate::app::myroomwakeupselectroot::MyRoomWakeupSelectRoot_CursorTop,
    ) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy() -> ();

    #[method(name = "GetWakeupSelectMenuContent", args = 0)]
    pub fn get_wakeup_select_menu_content(
        self,
    ) -> crate::app::myroomwakeupselectmenucontent::MyRoomWakeupSelectMenuContent;

    #[method(name = "CloseRelianceRank", args = 0)]
    pub fn close_reliance_rank(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomwakeupselectroot")]
impl MyRoomWakeupSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectRootMethods>::ctor(this);
        this
    }
}
