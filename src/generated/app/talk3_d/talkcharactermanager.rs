
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkcharactermanager/TalkCharacterManager_ProcFadeOutCharacter.md")))]
#[::unity2::class(
    namespace = "App.Talk3D",
    name = "TalkCharacterManager.ProcFadeOutCharacter"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct TalkCharacterManager_ProcFadeOutCharacter {
    #[rename(name = "m_talkCharacterController")]
    pub m_talk_character_controller:
        crate::app::talk3_d::talkcharactercontroller::TalkCharacterController,
    #[rename(name = "m_fadeTime")]
    pub m_fade_time: f32,
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
#[::unity2::methods]
impl TalkCharacterManager_ProcFadeOutCharacter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "StartFadeOutCharacter", args = 0)]
    pub fn start_fade_out_character(self) -> ();

    #[method(name = "IsFadingCharacter", args = 0)]
    pub fn is_fading_character(self) -> bool;

    #[method(name = "DeleteCharacter", args = 0)]
    pub fn delete_character(self) -> ();

    #[method(name = "Create", args = 3)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
impl TalkCharacterManager_ProcFadeOutCharacter {
    pub fn new(
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkCharacterManager_ProcFadeOutCharacter),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkCharacterManager_ProcFadeOutCharacterMethods>::ctor(
            this,
            talk_character_controller,
            fade_time,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkcharactermanager/TalkCharacterManager_onLoad.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkCharacterManager.onLoad")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct TalkCharacterManager_onLoad {}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
#[::unity2::methods]
impl TalkCharacterManager_onLoad {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, pid: ::unity2::Il2CppString, location_name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
impl TalkCharacterManager_onLoad {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkCharacterManager_onLoad),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkCharacterManager_onLoadMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkcharactermanager/TalkCharacterManager_ProcFadeInCharacter.md")))]
#[::unity2::class(
    namespace = "App.Talk3D",
    name = "TalkCharacterManager.ProcFadeInCharacter"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct TalkCharacterManager_ProcFadeInCharacter {
    #[rename(name = "m_talkCharacterController")]
    pub m_talk_character_controller:
        crate::app::talk3_d::talkcharactercontroller::TalkCharacterController,
    #[rename(name = "m_fadeTime")]
    pub m_fade_time: f32,
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
#[::unity2::methods]
impl TalkCharacterManager_ProcFadeInCharacter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "StartFadeInCharacter", args = 0)]
    pub fn start_fade_in_character(self) -> ();

    #[method(name = "IsFadingCharacter", args = 0)]
    pub fn is_fading_character(self) -> bool;

    #[method(name = "Create", args = 3)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> ();
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
impl TalkCharacterManager_ProcFadeInCharacter {
    pub fn new(
        talk_character_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
        fade_time: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkCharacterManager_ProcFadeInCharacter),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkCharacterManager_ProcFadeInCharacterMethods>::ctor(
            this,
            talk_character_controller,
            fade_time,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkcharactermanager/TalkCharacterManager.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkCharacterManager")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: talk3_d :: talkcharactermanager :: TalkCharacterManager >)]
pub struct TalkCharacterManager {
    #[static_field]
    #[rename(name = "PositionLocatorName")]
    pub position_locator_name: ::unity2::Il2CppString,
    #[rename(name = "m_FadeTime")]
    pub m_fade_time: f32,
    #[rename(name = "m_ChangeLookAtTime")]
    pub m_change_look_at_time: f32,
    #[rename(name = "m_IsPlayFirstTransition")]
    pub m_is_play_first_transition: bool,
    #[rename(name = "m_VoiceThresholdToLip")]
    pub m_voice_threshold_to_lip: f32,
    #[rename(name = "m_VoiceMagnificationToLip")]
    pub m_voice_magnification_to_lip: f32,
    #[rename(name = "m_PauseChangeCrossFadeTime")]
    pub m_pause_change_cross_fade_time: f32,
    #[rename(name = "m_ModelPoolObject")]
    pub m_model_pool_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StandPositions")]
    pub m_stand_positions: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StandRoot")]
    pub m_stand_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StandPositionRoot")]
    pub m_stand_position_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FaceRoot")]
    pub m_face_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DirectRoot")]
    pub m_direct_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FrameCanvas")]
    pub m_frame_canvas: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LookAtTargetRoot")]
    pub m_look_at_target_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TopFaceCameraObject")]
    pub m_top_face_camera_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BottomFaceCameraObject")]
    pub m_bottom_face_camera_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LoadPidList")]
    pub m_load_pid_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
#[::unity2::methods]
impl TalkCharacterManager {
    #[method(name = "get_TalkType", args = 0)]
    pub fn get_talk_type(self) -> crate::app::talk3_d::talk_2::Talk_TalkType;

    #[method(name = "get_ActivePid", args = 0)]
    pub fn get_active_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_IsPlayFirstTransition", args = 0)]
    pub fn get_is_play_first_transition(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetActiveToChildren", args = 2)]
    pub fn set_active_to_children(
        self,
        root: crate::unity_engine::gameobject::GameObject,
        is_active: bool,
    ) -> ();

    #[method(name = "SetTalkType", args = 2)]
    pub fn set_talk_type(
        self,
        talk_type: crate::app::talk3_d::talk_2::Talk_TalkType,
        positions_root_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PreLoadCharacter", args = 2)]
    pub fn pre_load_character(
        self,
        pid: ::unity2::Il2CppString,
        pid_for_create: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateCharacter", args = 3)]
    pub fn create_character(
        self,
        pid: ::unity2::Il2CppString,
        location_name: ::unity2::Il2CppString,
        callback: crate::app::talk3_d::talkcharactermanager::TalkCharacterManager_onLoad,
    ) -> ();

    #[method(name = "CreateCharacterImpl", args = 3)]
    pub fn create_character_impl(
        self,
        pid: ::unity2::Il2CppString,
        location_name: ::unity2::Il2CppString,
        callback: crate::app::talk3_d::talkcharactermanager::TalkCharacterManager_onLoad,
    ) -> ();

    #[method(name = "SetupCharactorForDisplay", args = 4)]
    pub fn setup_charactor_for_display(
        self,
        pid: ::unity2::Il2CppString,
        location_name: ::unity2::Il2CppString,
        chara: crate::combat::character::Character,
        locator: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "EnableFaceCamera", args = 1)]
    pub fn enable_face_camera(self, location_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SearchFromPool", args = 1)]
    pub fn search_from_pool(
        self,
        pid: ::unity2::Il2CppString,
    ) -> crate::combat::character::Character;

    #[method(name = "FadeInCharacter", args = 1)]
    pub fn fade_in_character(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "FadeOutCharacter", args = 1)]
    pub fn fade_out_character(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "DeleteCharacter", args = 1)]
    pub fn delete_character(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "DeleteAllCharacter", args = 0)]
    pub fn delete_all_character(self) -> ();

    #[method(name = "MoveModelToPool", args = 1)]
    pub fn move_model_to_pool(self, chara_obj: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 1)]
    pub fn show(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "Hide", args = 1)]
    pub fn hide(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "ChangeAnimeBody", args = 2)]
    pub fn change_anime_body(
        self,
        pid: ::unity2::Il2CppString,
        anime_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ChangeAnimeFace", args = 2)]
    pub fn change_anime_face(
        self,
        pid: ::unity2::Il2CppString,
        anime_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FindCharacter", args = 1)]
    pub fn find_character(self, pid: ::unity2::Il2CppString)
        -> crate::combat::character::Character;

    #[method(name = "GetCharaLocationName", args = 1)]
    pub fn get_chara_location_name(self, pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "FindLocatorByPID", args = 1)]
    pub fn find_locator_by_pid(
        self,
        pid: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindLocatorByLocationName", args = 1)]
    pub fn find_locator_by_location_name(
        self,
        location_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "IsFading", args = 0)]
    pub fn is_fading() -> bool;

    #[method(name = "PidToPidForCreate", args = 1)]
    pub fn pid_to_pid_for_create(pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkcharactermanager")]
impl TalkCharacterManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkCharacterManager),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkCharacterManagerMethods>::ctor(this);
        this
    }
}
