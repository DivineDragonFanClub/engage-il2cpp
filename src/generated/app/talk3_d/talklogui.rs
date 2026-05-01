
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogui/TalkLogUI_ScrollBar.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLogUI.ScrollBar")]
#[parent(crate::system::object::Object)]
pub struct TalkLogUI_ScrollBar {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ScrollBar")]
    pub m_scroll_bar: crate::unity_engine::ui::scrollbar::Scrollbar,
    #[rename(name = "m_ScrollAreaImage")]
    pub m_scroll_area_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SlideHandleImage")]
    pub m_slide_handle_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ItemMax")]
    pub m_item_max: i32,
    #[rename(name = "m_SlideHandlePosFrom")]
    pub m_slide_handle_pos_from: f32,
    #[rename(name = "m_SlideHandlePosTo")]
    pub m_slide_handle_pos_to: f32,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-talk3_d-talklogui")]
#[::unity2::methods]
impl TalkLogUI_ScrollBar {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        root_object: crate::unity_engine::gameobject::GameObject,
        item_max: i32,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "MoveSlideHandlePos", args = 3)]
    pub fn move_slide_handle_pos(self, scroll_index: f32, log_num: f32, msec: f32) -> ();

    #[method(name = "SetSlideHandleSize", args = 1)]
    pub fn set_slide_handle_size(self, log_num: f32) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-talk3_d-talklogui")]
impl TalkLogUI_ScrollBar {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject, item_max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLogUI_ScrollBar),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogUI_ScrollBarMethods>::ctor(this, root_object, item_max);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogui/TalkLogUI.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLogUI")]
#[parent(crate::system::object::Object)]
pub struct TalkLogUI {}

#[cfg(feature = "app-talk3_d-talklogui")]
#[::unity2::methods]
impl TalkLogUI {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talklogui")]
impl TalkLogUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLogUI),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogUIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogui/TalkLogUI_Cursor.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLogUI.Cursor")]
#[parent(crate::system::object::Object)]
pub struct TalkLogUI_Cursor {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PosFrom")]
    pub m_pos_from: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_PosTo")]
    pub m_pos_to: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-talk3_d-talklogui")]
#[::unity2::methods]
impl TalkLogUI_Cursor {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "MovePosition", args = 2)]
    pub fn move_position(
        self,
        win: crate::app::talk3_d::talklogui::TalkLogUI_Window,
        msec: f32,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talklogui")]
impl TalkLogUI_Cursor {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLogUI_Cursor),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogUI_CursorMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogui/TalkLogUI_Window.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLogUI.Window")]
#[parent(crate::system::object::Object)]
pub struct TalkLogUI_Window {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RootAnimator")]
    pub m_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_RootImage")]
    pub m_root_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NameWindowImage")]
    pub m_name_window_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_TalkerName")]
    pub m_talker_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BodyMess")]
    pub m_body_mess: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_KeyHelpMess")]
    pub m_key_help_mess: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_KeyHelpAnimator")]
    pub m_key_help_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_KeyHelpIconImage")]
    pub m_key_help_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_isMoveAnimStarted")]
    pub m_is_move_anim_started: bool,
    #[static_field]
    #[rename(name = "WinUpDownAnimSpeed")]
    pub win_up_down_anim_speed: f32,
}

#[cfg(feature = "app-talk3_d-talklogui")]
#[::unity2::methods]
impl TalkLogUI_Window {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, index: i32, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Update", args = 3)]
    pub fn update(
        self,
        is_selected: bool,
        is_voice_event_exist: bool,
        is_voice_event_playing: bool,
    ) -> ();

    #[method(name = "GetIndex", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "IsSelected", args = 4)]
    pub fn is_selected(
        self,
        log_add_count: i32,
        view_window_max: i32,
        cursor_index: i32,
        scroll_index: i32,
    ) -> bool;

    #[method(name = "GetTransform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "SetTalkerName", args = 1)]
    pub fn set_talker_name(self, talker_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetBodyMess", args = 1)]
    pub fn set_body_mess(self, body_mess: ::unity2::Il2CppString) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "ShowKeyHelp", args = 0)]
    pub fn show_key_help(self) -> ();

    #[method(name = "HideKeyHelp", args = 0)]
    pub fn hide_key_help(self) -> ();

    #[method(name = "PlayAnim_KeyHelpIdle", args = 0)]
    pub fn play_anim_key_help_idle(self) -> ();

    #[method(name = "PlayAnim_KeyHelpPlayingVoice", args = 0)]
    pub fn play_anim_key_help_playing_voice(self) -> ();

    #[method(name = "IsPlayingMoveAnimation", args = 0)]
    pub fn is_playing_move_animation(self) -> bool;

    #[method(name = "PlayUpAnim", args = 1)]
    pub fn play_up_anim(self, is_move_fast: bool) -> ();

    #[method(name = "PlayDownAnim", args = 1)]
    pub fn play_down_anim(self, is_move_fast: bool) -> ();

    #[method(name = "GetWindowUpDownAnimDurationMsec", args = 0)]
    pub fn get_window_up_down_anim_duration_msec(self) -> f32;
}

#[cfg(feature = "app-talk3_d-talklogui")]
impl TalkLogUI_Window {
    pub fn new(index: i32, root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLogUI_Window),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogUI_WindowMethods>::ctor(this, index, root_object);
        this
    }
}
