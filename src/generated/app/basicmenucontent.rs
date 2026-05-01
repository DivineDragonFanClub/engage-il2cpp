
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicmenucontent/BasicMenuContent_Scroll.md")))]
#[::unity2::class(namespace = "App", name = "BasicMenuContent.Scroll")]
#[parent(crate::system::object::Object)]
pub struct BasicMenuContent_Scroll {
    #[rename(name = "m_menuContent")]
    pub m_menu_content: crate::app::basicmenucontent::BasicMenuContent,
    #[rename(name = "m_objScrollBar")]
    pub m_obj_scroll_bar: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_scrollBar")]
    pub m_scroll_bar: crate::unity_engine::ui::scrollbar::Scrollbar,
    #[rename(name = "m_scrollNow")]
    pub m_scroll_now: f32,
    #[rename(name = "m_scrollOld")]
    pub m_scroll_old: f32,
    #[rename(name = "m_scrollFrom")]
    pub m_scroll_from: f32,
    #[rename(name = "m_scrollTick")]
    pub m_scroll_tick: f32,
    #[rename(name = "m_scrollFrame")]
    pub m_scroll_frame: f32,
}

#[cfg(feature = "app-basicmenucontent")]
#[::unity2::methods]
impl BasicMenuContent_Scroll {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        scroll_bar: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetScroll", args = 0)]
    pub fn get_scroll(self) -> f32;

    #[method(name = "GetScrollOld", args = 0)]
    pub fn get_scroll_old(self) -> f32;

    #[method(name = "GetScrollFrom", args = 0)]
    pub fn get_scroll_from(self) -> f32;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "ScrollInstant", args = 0)]
    pub fn scroll_instant(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "SetScrollFrame", args = 1)]
    pub fn set_scroll_frame(self, frame: f32) -> ();

    #[method(name = "SetScrollBarHandleSize", args = 1)]
    pub fn set_scroll_bar_handle_size(self, size: f32) -> ();
}

#[cfg(feature = "app-basicmenucontent")]
impl BasicMenuContent_Scroll {
    pub fn new(
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        scroll_bar: crate::unity_engine::gameobject::GameObject,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuContent_Scroll),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicMenuContent_ScrollMethods>::ctor(this, menu_content, scroll_bar);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicmenucontent/BasicMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "BasicMenuContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BasicMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_srcMaterial")]
    pub m_src_material: crate::unity_engine::material::Material,
    #[rename(name = "m_material")]
    pub m_material: crate::unity_engine::material::Material,
    #[rename(name = "m_menu")]
    pub m_menu: crate::app::basicmenu::BasicMenu,
    #[rename(name = "m_winAnimator")]
    pub m_win_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_objMenu")]
    pub m_obj_menu: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objFrontCursor")]
    pub m_obj_front_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objBackCursor")]
    pub m_obj_back_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objViewport")]
    pub m_obj_viewport: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objVerticalScrollBar")]
    pub m_obj_vertical_scroll_bar: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objScrollBarHandle")]
    pub m_obj_scroll_bar_handle: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objContent")]
    pub m_obj_content: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objSubMenuBase")]
    pub m_obj_sub_menu_base: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_backContent")]
    pub m_back_content: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_cursor")]
    pub m_cursor: crate::app::basicmenucontent::BasicMenuContent_Cursor,
    #[rename(name = "m_scroll")]
    pub m_scroll: crate::app::basicmenucontent::BasicMenuContent_Scroll,
    #[rename(name = "m_pos")]
    pub m_pos: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_posOld")]
    pub m_pos_old: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_anchorType")]
    pub m_anchor_type: crate::app::basicmenu::BasicMenu_AnchorType,
    #[rename(name = "m_anchoredPosOriginal")]
    pub m_anchored_pos_original: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_anchorMinOriginal")]
    pub m_anchor_min_original: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_anchorMaxOriginal")]
    pub m_anchor_max_original: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_objContentBaseLocalPos")]
    pub m_obj_content_base_local_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_backContentBaseLocalPos")]
    pub m_back_content_base_local_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_requestAdjust")]
    pub m_request_adjust: bool,
}

#[cfg(feature = "app-basicmenucontent")]
#[::unity2::methods]
impl BasicMenuContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "GetRectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "GetMenu", args = 0)]
    pub fn get_menu(self) -> crate::app::basicmenu::BasicMenu;

    #[method(name = "SetMenu", args = 1)]
    pub fn set_menu(self, menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "SetAnimator", args = 1)]
    pub fn set_animator(self, anim: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorWidth", args = 1)]
    pub fn calc_cursor_width(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorHeight", args = 1)]
    pub fn calc_cursor_height(self, menu_item_index: i32) -> f32;

    #[method(name = "NowCursorWidth", args = 0)]
    pub fn now_cursor_width(self) -> f32;

    #[method(name = "NowCursorHeight", args = 0)]
    pub fn now_cursor_height(self) -> f32;

    #[method(name = "GetScreenScale", args = 0)]
    pub fn get_screen_scale(self) -> f32;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "SetCursorMoveFrame", args = 1)]
    pub fn set_cursor_move_frame(self, frame: f32) -> ();

    #[method(name = "CursorMoveInstant", args = 0)]
    pub fn cursor_move_instant(self) -> ();

    #[method(name = "SetScrollFrame", args = 1)]
    pub fn set_scroll_frame(self, frame: f32) -> ();

    #[method(name = "SetScrollBarHandleSize", args = 1)]
    pub fn set_scroll_bar_handle_size(self, size: f32) -> ();

    #[method(name = "IsScrollBusy", args = 0)]
    pub fn is_scroll_busy(self) -> bool;

    #[method(name = "ScrollInstant", args = 0)]
    pub fn scroll_instant(self) -> ();

    #[method(name = "ResetScroll", args = 0)]
    pub fn reset_scroll(self) -> ();

    #[method(name = "SetCursorColor", args = 1)]
    pub fn set_cursor_color(self, c: crate::unity_engine::color::Color) -> ();

    #[method(name = "PutCursorInFront", args = 0)]
    pub fn put_cursor_in_front(self) -> ();

    #[method(name = "PutCursorInBack", args = 0)]
    pub fn put_cursor_in_back(self) -> ();

    #[method(name = "SetCursorToKeepAnimatorState", args = 1)]
    pub fn set_cursor_to_keep_animator_state(self, keep: bool) -> ();

    #[method(name = "MoveFrontCursorFrom", args = 3)]
    pub fn move_front_cursor_from(self, from_x: f32, from_y: f32, frame: f32) -> ();

    #[method(name = "MoveFrontCursorFrom", args = 2)]
    pub fn move_front_cursor_from_2(
        self,
        from_menu_item: crate::app::basicmenuitem::BasicMenuItem,
        frame: f32,
    ) -> ();

    #[method(name = "SetFrontCursorVisibility", args = 1)]
    pub fn set_front_cursor_visibility(self, visibility: bool) -> ();

    #[method(name = "RestartCursorAnim", args = 0)]
    pub fn restart_cursor_anim(self) -> ();

    #[method(name = "GetSubMenuBaseTransform", args = 0)]
    pub fn get_sub_menu_base_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "Suspend", args = 0)]
    pub fn suspend(self) -> ();

    #[method(name = "UnSuspend", args = 0)]
    pub fn un_suspend(self) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "BuildMaterial", args = 0)]
    pub fn build_material(self) -> ();

    #[method(name = "BuildWH", args = 0)]
    pub fn build_wh(self) -> ();

    #[method(name = "BuildContentPos", args = 0)]
    pub fn build_content_pos(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "GetCursorOffsetX", args = 0)]
    pub fn get_cursor_offset_x(self) -> f32;

    #[method(name = "GetCursorOffsetY", args = 0)]
    pub fn get_cursor_offset_y(self) -> f32;

    #[method(name = "OpenAnime", args = 0)]
    pub fn open_anime(self) -> ();

    #[method(name = "CloseAnime", args = 0)]
    pub fn close_anime(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "SetPivot", args = 2)]
    pub fn set_pivot(self, x: f32, y: f32) -> ();

    #[method(name = "GetAnchorType", args = 0)]
    pub fn get_anchor_type(self) -> crate::app::basicmenu::BasicMenu_AnchorType;

    #[method(name = "SetAnchorType", args = 3)]
    pub fn set_anchor_type(
        self,
        anchor_type: crate::app::basicmenu::BasicMenu_AnchorType,
        x: f32,
        y: f32,
    ) -> ();

    #[method(name = "SetAnchorTypeFromScreenCoord", args = 3)]
    pub fn set_anchor_type_from_screen_coord(
        self,
        anchor_type: crate::app::basicmenu::BasicMenu_AnchorType,
        x: f32,
        y: f32,
    ) -> ();

    #[method(name = "AnchorPositionToAnchorType", args = 2)]
    pub fn anchor_position_to_anchor_type(
        x: f32,
        y: f32,
    ) -> crate::app::basicmenu::BasicMenu_AnchorType;

    #[method(name = "ScreenPointToLocalPoint", args = 2)]
    pub fn screen_point_to_local_point(
        self,
        x: f32,
        y: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetX", args = 0)]
    pub fn get_x(self) -> f32;

    #[method(name = "GetY", args = 0)]
    pub fn get_y(self) -> f32;

    #[method(name = "GetPos", args = 0)]
    pub fn get_pos(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SetX", args = 1)]
    pub fn set_x(self, x: f32) -> ();

    #[method(name = "SetY", args = 1)]
    pub fn set_y(self, y: f32) -> ();

    #[method(name = "SetPos", args = 2)]
    pub fn set_pos(self, x: f32, y: f32) -> ();

    #[method(name = "SetTransformAsSubMenu", args = 2)]
    pub fn set_transform_as_sub_menu(
        self,
        parent_menu: crate::app::basicmenu::BasicMenu,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();

    #[method(name = "UpdateX", args = 0)]
    pub fn update_x(self) -> ();

    #[method(name = "UpdateY", args = 0)]
    pub fn update_y(self) -> ();

    #[method(name = "UpdatePos", args = 0)]
    pub fn update_pos(self) -> ();

    #[method(name = "GetW", args = 0)]
    pub fn get_w(self) -> f32;

    #[method(name = "GetH", args = 0)]
    pub fn get_h(self) -> f32;

    #[method(name = "UpdateAfterScroll", args = 1)]
    pub fn update_after_scroll(self, is_force_update: bool) -> ();

    #[method(name = "ForceRebuildLayout", args = 0)]
    pub fn force_rebuild_layout(self) -> ();

    #[method(name = "CycleMenuItemContent", args = 2)]
    pub fn cycle_menu_item_content(self, is_forward: bool, cycle_count: i32) -> ();

    #[method(name = "GetLineHeightForScroll", args = 0)]
    pub fn get_line_height_for_scroll(self) -> f32;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "CalcName", args = 2)]
    pub fn calc_name(
        menu_name: ::unity2::Il2CppString,
        asset_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::basicmenucontent::BasicMenuContent;

    #[method(name = "LoadPrefabAsync", args = 1)]
    pub fn load_prefab_async_2(path: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadPrefab", args = 1)]
    pub fn unload_prefab(path: ::unity2::Il2CppString) -> ();

    #[method(name = "IsLoadingPrefab", args = 1)]
    pub fn is_loading_prefab(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetCanvas", args = 0)]
    pub fn get_canvas() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-basicmenucontent")]
impl BasicMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicmenucontent/BasicMenuContent_Cursor.md")))]
#[::unity2::class(namespace = "App", name = "BasicMenuContent.Cursor")]
#[parent(crate::system::object::Object)]
pub struct BasicMenuContent_Cursor {
    #[rename(name = "m_menuContent")]
    pub m_menu_content: crate::app::basicmenucontent::BasicMenuContent,
    #[rename(name = "m_objFrontCursor")]
    pub m_obj_front_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objBackCursor")]
    pub m_obj_back_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_objFrontCursorBase")]
    pub m_obj_front_cursor_base: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_backCursorRect")]
    pub m_back_cursor_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_backCursorImage")]
    pub m_back_cursor_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_backCursorAnimator")]
    pub m_back_cursor_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_frontCursorAnimator")]
    pub m_front_cursor_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_frontCursorBaseRect")]
    pub m_front_cursor_base_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_frontCursorBaseParentRect")]
    pub m_front_cursor_base_parent_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_posX")]
    pub m_pos_x: f32,
    #[rename(name = "m_posY")]
    pub m_pos_y: f32,
    #[rename(name = "m_fromX")]
    pub m_from_x: f32,
    #[rename(name = "m_fromY")]
    pub m_from_y: f32,
    #[rename(name = "m_fromW")]
    pub m_from_w: f32,
    #[rename(name = "m_fromH")]
    pub m_from_h: f32,
    #[rename(name = "m_moveTick")]
    pub m_move_tick: f32,
    #[rename(name = "m_moveFrame")]
    pub m_move_frame: f32,
    #[rename(name = "m_fixed")]
    pub m_fixed: bool,
    #[rename(name = "m_fixedWH")]
    pub m_fixed_wh: bool,
    #[rename(name = "m_firstTick")]
    pub m_first_tick: bool,
    #[rename(name = "m_lastVisibility")]
    pub m_last_visibility: bool,
    #[rename(name = "m_frontCursorVisibility")]
    pub m_front_cursor_visibility: bool,
    #[rename(name = "m_frontCursorMoveTick")]
    pub m_front_cursor_move_tick: f32,
    #[rename(name = "m_frontCursorMoveFrame")]
    pub m_front_cursor_move_frame: f32,
    #[rename(name = "m_frontCursorFromX")]
    pub m_front_cursor_from_x: f32,
    #[rename(name = "m_frontCursorFromY")]
    pub m_front_cursor_from_y: f32,
}

#[cfg(feature = "app-basicmenucontent")]
#[::unity2::methods]
impl BasicMenuContent_Cursor {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        obj_front_cursor: crate::unity_engine::gameobject::GameObject,
        obj_back_cursor: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetPosX", args = 0)]
    pub fn get_pos_x(self) -> f32;

    #[method(name = "GetPosY", args = 0)]
    pub fn get_pos_y(self) -> f32;

    #[method(name = "Tick", args = 1)]
    pub fn tick(self, scroll: crate::app::basicmenucontent::BasicMenuContent_Scroll) -> ();

    #[method(name = "UpdateVisibility", args = 0)]
    pub fn update_visibility(self) -> ();

    #[method(name = "MoveInstant", args = 0)]
    pub fn move_instant(self) -> ();

    #[method(name = "SetMoveFrame", args = 1)]
    pub fn set_move_frame(self, frame: f32) -> ();

    #[method(name = "PauseAnim", args = 0)]
    pub fn pause_anim(self) -> ();

    #[method(name = "ResumeAnim", args = 0)]
    pub fn resume_anim(self) -> ();

    #[method(name = "RestartAnim", args = 0)]
    pub fn restart_anim(self) -> ();

    #[method(name = "SetSiblingIndex", args = 1)]
    pub fn set_sibling_index(self, index: i32) -> ();

    #[method(name = "SetBackCursorImageEnabled", args = 1)]
    pub fn set_back_cursor_image_enabled(self, enabled: bool) -> ();

    #[method(name = "SetToKeepAnimatorState", args = 1)]
    pub fn set_to_keep_animator_state(self, keep: bool) -> ();

    #[method(name = "MoveFrontCursorFrom", args = 3)]
    pub fn move_front_cursor_from(self, from_x: f32, from_y: f32, frame: f32) -> ();

    #[method(name = "SetFrontCursorVisibility", args = 1)]
    pub fn set_front_cursor_visibility(self, visibility: bool) -> ();
}

#[cfg(feature = "app-basicmenucontent")]
impl BasicMenuContent_Cursor {
    pub fn new(
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        obj_front_cursor: crate::unity_engine::gameobject::GameObject,
        obj_back_cursor: crate::unity_engine::gameobject::GameObject,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuContent_Cursor),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicMenuContent_CursorMethods>::ctor(
            this,
            menu_content,
            obj_front_cursor,
            obj_back_cursor,
        );
        this
    }
}
