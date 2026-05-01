
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: talk3_d :: talkui :: TalkUI >)]
pub struct TalkUI {
    #[static_field]
    #[rename(name = "EventPictureMax")]
    pub event_picture_max: i32,
    #[static_field]
    #[rename(name = "FaceLocationName_Left")]
    pub face_location_name_left: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FaceLocationName_Right")]
    pub face_location_name_right: ::unity2::Il2CppString,
    #[rename(name = "m_SystemObjects")]
    pub m_system_objects: crate::app::talk3_d::talkui::TalkUI_SystemObjects,
    #[rename(name = "m_StandObjects")]
    pub m_stand_objects: crate::app::talk3_d::talkui::TalkUI_StandObjects,
    #[rename(name = "m_FaceObjects")]
    pub m_face_objects: crate::app::talk3_d::talkui::TalkUI_FaceObjects,
    #[rename(name = "m_FocusTalkObjects")]
    pub m_focus_talk_objects: crate::app::talk3_d::talkui::TalkUI_TalkObjects,
    #[rename(name = "m_ReserveFocusWindow")]
    pub m_reserve_focus_window: ::unity2::Il2CppString,
    #[rename(name = "m_EventPictureController")]
    pub m_event_picture_controller:
        ::unity2::Array<crate::app::eventpicturecontroller::EventPictureController>,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI {
    #[method(name = "get_TalkType", args = 0)]
    pub fn get_talk_type(self) -> crate::app::talk3_d::talk_2::Talk_TalkType;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "ResetUI", args = 0)]
    pub fn reset_ui(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "ShowWindowBg", args = 0)]
    pub fn show_window_bg(self) -> ();

    #[method(name = "HideWindowBg", args = 0)]
    pub fn hide_window_bg(self) -> ();

    #[method(name = "SetTalkType", args = 2)]
    pub fn set_talk_type(
        self,
        talk_type: crate::app::talk3_d::talk_2::Talk_TalkType,
        name_plate_root_object_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ReOpenWindow", args = 0)]
    pub fn re_open_window(self) -> ();

    #[method(name = "AddLetterToFocusWindow", args = 1)]
    pub fn add_letter_to_focus_window(self, chr: u16) -> bool;

    #[method(name = "AddStringToFocusWindow", args = 1)]
    pub fn add_string_to_focus_window(self, str: ::unity2::Il2CppString) -> bool;

    #[method(name = "ClearForcusWindowText", args = 0)]
    pub fn clear_forcus_window_text(self) -> ();

    #[method(name = "ShowKeyWaitIcon", args = 0)]
    pub fn show_key_wait_icon(self) -> ();

    #[method(name = "HideKeyWaitIcon", args = 0)]
    pub fn hide_key_wait_icon(self) -> ();

    #[method(name = "SetupKeyHelp", args = 0)]
    pub fn setup_key_help(self) -> ();

    #[method(name = "SetupWaitIcon", args = 0)]
    pub fn setup_wait_icon(self) -> ();

    #[method(name = "SetTalkerName", args = 2)]
    pub fn set_talker_name(
        self,
        name: ::unity2::Il2CppString,
        location_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CloseTalkerName", args = 0)]
    pub fn close_talker_name(self) -> ();

    #[method(name = "FadeInFace", args = 1)]
    pub fn fade_in_face(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "FadeOutFace", args = 1)]
    pub fn fade_out_face(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetFaceSlideRate", args = 2)]
    pub fn set_face_slide_rate(
        self,
        location_name: ::unity2::Il2CppString,
        face_slide_rate: f32,
    ) -> ();

    #[method(name = "ChangeFocus", args = 1)]
    pub fn change_focus(self, location: ::unity2::Il2CppString) -> ();

    #[method(name = "EnableFaceSilhouette", args = 1)]
    pub fn enable_face_silhouette(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "DisableFaceSilhouette", args = 1)]
    pub fn disable_face_silhouette(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all(self) -> ();

    #[method(name = "IsPlayingAnimation", args = 0)]
    pub fn is_playing_animation(self) -> bool;

    #[method(name = "StartPageScroll", args = 0)]
    pub fn start_page_scroll(self) -> ();

    #[method(name = "IsPageScrolling", args = 0)]
    pub fn is_page_scrolling(self) -> bool;

    #[method(name = "IsFocusWindowTextEmpty", args = 0)]
    pub fn is_focus_window_text_empty(self) -> bool;

    #[method(name = "AddToLog", args = 2)]
    pub fn add_to_log(
        self,
        label: ::unity2::Il2CppString,
        talker_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ResetReserveFocus", args = 0)]
    pub fn reset_reserve_focus(self) -> ();

    #[method(name = "ReserveFocus", args = 1)]
    pub fn reserve_focus(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CheckReserveFocus", args = 0)]
    pub fn check_reserve_focus(self) -> ();

    #[method(name = "ShowPicture", args = 3)]
    pub fn show_picture(
        self,
        event_picture_index: i32,
        texture_name: ::unity2::Il2CppString,
        anim_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "HidePicture", args = 2)]
    pub fn hide_picture(self, event_picture_index: i32, anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetNamePlateLocator", args = 1)]
    pub fn get_name_plate_locator(
        self,
        talk_chara_controller_object: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CalcTotalWidth", args = 1)]
    pub fn calc_total_width(self, mess_str: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_SystemObjects.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.SystemObjects")]
#[parent(crate::app::talk3_d::talkui::TalkUI_TalkObjects)]
pub struct TalkUI_SystemObjects {}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_SystemObjects {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "AddToLog", args = 2)]
    pub fn add_to_log(
        self,
        label: ::unity2::Il2CppString,
        talker_name: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_SystemObjects {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_SystemObjects),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_SystemObjectsMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_KeyHelpObjects.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.KeyHelpObjects")]
#[parent(crate::system::object::Object)]
pub struct TalkUI_KeyHelpObjects {
    #[rename(name = "m_rootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_talkObjects")]
    pub m_talk_objects: crate::app::talk3_d::talkui::TalkUI_TalkObjects,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_KeyHelpObjects {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, game_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetTalkObjects", args = 1)]
    pub fn set_talk_objects(
        self,
        talk_objects: crate::app::talk3_d::talkui::TalkUI_TalkObjects,
    ) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_KeyHelpObjects {
    pub fn new(game_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_KeyHelpObjects),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_KeyHelpObjectsMethods>::ctor(this, game_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_FaceObjects_Face.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.FaceObjects.Face")]
#[parent(crate::system::object::Object)]
pub struct TalkUI_FaceObjects_Face {
    #[static_field]
    #[rename(name = "FaceSlideLength")]
    pub face_slide_length: f32,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_ImageBaseX")]
    pub m_image_base_x: f32,
    #[rename(name = "m_IsDirRight")]
    pub m_is_dir_right: bool,
    #[rename(name = "m_material")]
    pub m_material: crate::unity_engine::material::Material,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_FaceObjects_Face {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        root_object: crate::unity_engine::gameobject::GameObject,
        image_name: ::unity2::Il2CppString,
        is_dir_right: bool,
    ) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "FadeOut", args = 0)]
    pub fn fade_out(self) -> ();

    #[method(name = "SetSlideRate", args = 1)]
    pub fn set_slide_rate(self, face_slide_rate: f32) -> ();

    #[method(name = "EnableSilhouette", args = 0)]
    pub fn enable_silhouette(self) -> ();

    #[method(name = "DisableSilhouette", args = 0)]
    pub fn disable_silhouette(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "ResetAnim", args = 0)]
    pub fn reset_anim(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_FaceObjects_Face {
    pub fn new(
        root_object: crate::unity_engine::gameobject::GameObject,
        image_name: ::unity2::Il2CppString,
        is_dir_right: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_FaceObjects_Face),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_FaceObjects_FaceMethods>::ctor(
            this,
            root_object,
            image_name,
            is_dir_right,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_StandObjects.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.StandObjects")]
#[parent(crate::app::talk3_d::talkui::TalkUI_SystemObjects)]
pub struct TalkUI_StandObjects {
    #[rename(name = "m_NameRoot")]
    pub m_name_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CurrentLocationName")]
    pub m_current_location_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_StandObjects {
    #[method(name = "GetCurrentLocationName", args = 0)]
    pub fn get_current_location_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "SetTalkerName", args = 2)]
    pub fn set_talker_name(
        self,
        name: ::unity2::Il2CppString,
        location_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetFocus", args = 1)]
    pub fn set_focus(self, location_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "CloseTalkerName", args = 0)]
    pub fn close_talker_name(self) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all(self) -> ();

    #[method(name = "GetNameLocator", args = 1)]
    pub fn get_name_locator(
        self,
        location_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "IsPlayingWindowAnimation", args = 0)]
    pub fn is_playing_window_animation(self) -> bool;

    #[method(name = "AddToLog", args = 2)]
    pub fn add_to_log(
        self,
        label: ::unity2::Il2CppString,
        talker_name: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_StandObjects {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_StandObjects),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_StandObjectsMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_FaceObjects.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.FaceObjects")]
#[parent(crate::app::talk3_d::talkui::TalkUI_StandObjects)]
pub struct TalkUI_FaceObjects {
    #[rename(name = "m_Panel")]
    pub m_panel: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PanelAnimator")]
    pub m_panel_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_FaceL")]
    pub m_face_l: crate::app::talk3_d::talkui::TalkUI_FaceObjects_Face,
    #[rename(name = "m_FaceR")]
    pub m_face_r: crate::app::talk3_d::talkui::TalkUI_FaceObjects_Face,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_FaceObjects {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "FadeInFaceL", args = 0)]
    pub fn fade_in_face_l(self) -> ();

    #[method(name = "FadeOutFaceL", args = 0)]
    pub fn fade_out_face_l(self) -> ();

    #[method(name = "SetFaceSlideRateL", args = 1)]
    pub fn set_face_slide_rate_l(self, face_slide_rate: f32) -> ();

    #[method(name = "EnableSilhouetteL", args = 0)]
    pub fn enable_silhouette_l(self) -> ();

    #[method(name = "DisableSilhouetteL", args = 0)]
    pub fn disable_silhouette_l(self) -> ();

    #[method(name = "FadeInFaceR", args = 0)]
    pub fn fade_in_face_r(self) -> ();

    #[method(name = "FadeOutFaceR", args = 0)]
    pub fn fade_out_face_r(self) -> ();

    #[method(name = "SetFaceSlideRateR", args = 1)]
    pub fn set_face_slide_rate_r(self, face_slide_rate: f32) -> ();

    #[method(name = "EnableSilhouetteR", args = 0)]
    pub fn enable_silhouette_r(self) -> ();

    #[method(name = "DisableSilhouetteR", args = 0)]
    pub fn disable_silhouette_r(self) -> ();

    #[method(name = "OpenPanel", args = 0)]
    pub fn open_panel(self) -> ();

    #[method(name = "ClosePanel", args = 0)]
    pub fn close_panel(self) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_FaceObjects {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_FaceObjects),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_FaceObjectsMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkui/TalkUI_TalkObjects.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUI.TalkObjects")]
#[parent(crate::system::object::Object)]
pub struct TalkUI_TalkObjects {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Window")]
    pub m_window: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WindowImage")]
    pub m_window_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_WindowAnimator")]
    pub m_window_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_MainTextAnimator")]
    pub m_main_text_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_MainText")]
    pub m_main_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_WaitKeyIcon")]
    pub m_wait_key_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AutoPlayIcon")]
    pub m_auto_play_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KeyHelpObjects")]
    pub m_key_help_objects: crate::app::talk3_d::talkui::TalkUI_KeyHelpObjects,
    #[rename(name = "m_ImmediatelyAfterActivation")]
    pub m_immediately_after_activation: bool,
    #[rename(name = "m_IsShowKeyWaitIcon")]
    pub m_is_show_key_wait_icon: bool,
}

#[cfg(feature = "app-talk3_d-talkui")]
#[::unity2::methods]
impl TalkUI_TalkObjects {
    #[method(name = "GetWindowAnimator", args = 0)]
    pub fn get_window_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "SetImmediatelyAfterActivation", args = 0)]
    pub fn set_immediately_after_activation(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "IsTextEmpty", args = 0)]
    pub fn is_text_empty(self) -> bool;

    #[method(name = "IsNeedTextScroll", args = 0)]
    pub fn is_need_text_scroll(self) -> bool;

    #[method(name = "AddLetter", args = 1)]
    pub fn add_letter(self, c: u16) -> ();

    #[method(name = "AddString", args = 1)]
    pub fn add_string(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "ClearText", args = 0)]
    pub fn clear_text(self) -> ();

    #[method(name = "CalcTotalWidth", args = 1)]
    pub fn calc_total_width(self, mess_str: ::unity2::Il2CppString) -> f32;

    #[method(name = "SetupWaitIcon", args = 0)]
    pub fn setup_wait_icon(self) -> ();

    #[method(name = "ShowKeyWaitIcon", args = 0)]
    pub fn show_key_wait_icon(self) -> ();

    #[method(name = "HideKeyWaitIcon", args = 0)]
    pub fn hide_key_wait_icon(self) -> ();

    #[method(name = "GetNameLocator", args = 1)]
    pub fn get_name_locator(
        self,
        location_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetTalkerName", args = 2)]
    pub fn set_talker_name(
        self,
        name: ::unity2::Il2CppString,
        location: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetFocus", args = 1)]
    pub fn set_focus(self, location: ::unity2::Il2CppString) -> bool;

    #[method(name = "CloseTalkerName", args = 0)]
    pub fn close_talker_name(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, is_active: bool) -> ();

    #[method(name = "OpenWindow", args = 0)]
    pub fn open_window(self) -> ();

    #[method(name = "CloseWindow", args = 0)]
    pub fn close_window(self) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all(self) -> ();

    #[method(name = "ShowWindowBg", args = 0)]
    pub fn show_window_bg(self) -> ();

    #[method(name = "HideWindowBg", args = 0)]
    pub fn hide_window_bg(self) -> ();

    #[method(name = "IsPlayingWindowAnimation", args = 0)]
    pub fn is_playing_window_animation(self) -> bool;

    #[method(name = "TryPlayAnime", args = 2)]
    pub fn try_play_anime(
        animator: crate::unity_engine::animator::Animator,
        state_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TryPlayOpen", args = 1)]
    pub fn try_play_open(animator: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "TryPlayClose", args = 1)]
    pub fn try_play_close(animator: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "StartTextScroll", args = 0)]
    pub fn start_text_scroll(self) -> ();

    #[method(name = "IsTextScrolling", args = 0)]
    pub fn is_text_scrolling(self) -> bool;

    #[method(name = "SetupKeyHelp", args = 0)]
    pub fn setup_key_help(self) -> ();

    #[method(name = "AddToLog", args = 2)]
    pub fn add_to_log(
        self,
        label: ::unity2::Il2CppString,
        talker_name: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talkui")]
impl TalkUI_TalkObjects {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUI_TalkObjects),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUI_TalkObjectsMethods>::ctor(this, root_object);
        this
    }
}
