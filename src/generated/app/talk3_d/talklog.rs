
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklog/TalkLog.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLog")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: talk3_d :: talklog :: TalkLog >)]
pub struct TalkLog {
    #[rename(name = "m_Bases")]
    pub m_bases: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
    #[rename(name = "m_LogPageMax")]
    pub m_log_page_max: i32,
    #[static_field]
    #[rename(name = "ViewWindowMax")]
    pub view_window_max: i32,
    #[rename(name = "m_Cursor")]
    pub m_cursor: crate::app::talk3_d::talklogui::TalkLogUI_Cursor,
    #[rename(name = "m_WindowList")]
    pub m_window_list: ::unity2::Array<crate::app::talk3_d::talklogui::TalkLogUI_Window>,
    #[rename(name = "m_ScrollBar")]
    pub m_scroll_bar: crate::app::talk3_d::talklogui::TalkLogUI_ScrollBar,
    #[rename(name = "m_LabelList")]
    pub m_label_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_TalkerNameList")]
    pub m_talker_name_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_MessList")]
    pub m_mess_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_VoiceEventList")]
    pub m_voice_event_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_ScenarioSoundBankNameList")]
    pub m_scenario_sound_bank_name_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_LogAddCount")]
    pub m_log_add_count: i32,
    #[rename(name = "m_LogNum")]
    pub m_log_num: i32,
    #[rename(name = "m_CursorIndex")]
    pub m_cursor_index: i32,
    #[rename(name = "m_ScrollIndex")]
    pub m_scroll_index: i32,
}

#[cfg(feature = "app-talk3_d-talklog")]
#[::unity2::methods]
impl TalkLog {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "ReBuild", args = 0)]
    pub fn re_build(self) -> ();

    #[method(name = "BuildObjects", args = 0)]
    pub fn build_objects(self) -> ();

    #[method(name = "ResetParam", args = 0)]
    pub fn reset_param(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LoadVoiceSoundBank", args = 0)]
    pub fn load_voice_sound_bank(self) -> ();

    #[method(name = "UnloadVoiceSoundBank", args = 0)]
    pub fn unload_voice_sound_bank(self) -> ();

    #[method(name = "ResetCursorAndScroll", args = 0)]
    pub fn reset_cursor_and_scroll(self) -> ();

    #[method(name = "SetupLayout", args = 0)]
    pub fn setup_layout(self) -> ();

    #[method(name = "SetupWindow", args = 0)]
    pub fn setup_window(self) -> ();

    #[method(name = "SetupCursor", args = 0)]
    pub fn setup_cursor(self) -> ();

    #[method(name = "SetupScrollBar", args = 0)]
    pub fn setup_scroll_bar(self) -> ();

    #[method(name = "CursorUp", args = 1)]
    pub fn cursor_up(self, is_move_fast: bool) -> ();

    #[method(name = "CursorDown", args = 2)]
    pub fn cursor_down(self, is_move_fast: bool, is_key_trigger: bool) -> bool;

    #[method(name = "ScrollUp", args = 2)]
    pub fn scroll_up(self, is_move_fast: bool, is_layout_update: bool) -> ();

    #[method(name = "ScrollDown", args = 2)]
    pub fn scroll_down(self, is_move_fast: bool, is_layout_update: bool) -> ();

    #[method(name = "AddLog", args = 3)]
    pub fn add_log(
        self,
        label: ::unity2::Il2CppString,
        mess: ::unity2::Il2CppString,
        talker: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ResetLogImpl", args = 0)]
    pub fn reset_log_impl(self) -> ();

    #[method(name = "IsPlayingWindowAnimation", args = 0)]
    pub fn is_playing_window_animation(self) -> bool;

    #[method(name = "GetWindowUpDownAnimDurationMsec", args = 0)]
    pub fn get_window_up_down_anim_duration_msec(self) -> f32;

    #[method(name = "IsVoiceEventExist", args = 1)]
    pub fn is_voice_event_exist(self, mess_index: i32) -> bool;

    #[method(name = "IsVoiceEventPlaying", args = 1)]
    pub fn is_voice_event_playing(self, mess_index: i32) -> bool;

    #[method(name = "TryPlayVoice", args = 0)]
    pub fn try_play_voice(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "ResetLog", args = 0)]
    pub fn reset_log() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talklog")]
impl TalkLog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLog),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogMethods>::ctor(this);
        this
    }
}
