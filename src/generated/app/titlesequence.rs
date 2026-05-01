
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlesequence/TitleSequence_ProcTitleCall_TitleCallData.md")))]
#[::unity2::class(namespace = "App", name = "TitleSequence.ProcTitleCall.TitleCallData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: titlesequence :: TitleSequence_ProcTitleCall_TitleCallData >)]
pub struct TitleSequence_ProcTitleCall_TitleCallData {}

#[cfg(feature = "app-titlesequence")]
#[::unity2::methods]
impl TitleSequence_ProcTitleCall_TitleCallData {
    #[method(name = "get_PidOrGid", args = 0)]
    pub fn get_pid_or_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PidOrGid", args = 1)]
    pub fn set_pid_or_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();
}

#[cfg(feature = "app-titlesequence")]
impl TitleSequence_ProcTitleCall_TitleCallData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleSequence_ProcTitleCall_TitleCallData),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleSequence_ProcTitleCall_TitleCallDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlesequence/TitleSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TitleSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TitleSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TitleSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TitleSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TitleSequence_Label {
    pub fn start() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlesequence/TitleSequence_ProcTitleCall.md")))]
#[::unity2::class(namespace = "App", name = "TitleSequence.ProcTitleCall")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: titlesequence :: TitleSequence_ProcTitleCall >)]
pub struct TitleSequence_ProcTitleCall {
    #[rename(name = "m_PidOrGid")]
    pub m_pid_or_gid: ::unity2::Il2CppString,
    #[rename(name = "m_IsHeroFemale")]
    pub m_is_hero_female: bool,
}

#[cfg(feature = "app-titlesequence")]
#[::unity2::methods]
impl TitleSequence_ProcTitleCall {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CalcPidOrGid", args = 0)]
    pub fn calc_pid_or_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "PlayTitleVoice", args = 0)]
    pub fn play_title_voice(self) -> ();

    #[method(name = "PlaySubtitleVoice", args = 0)]
    pub fn play_subtitle_voice(self) -> ();

    #[method(name = "PlayVoiceCommon", args = 2)]
    pub fn play_voice_common(
        self,
        sound_event_name: ::unity2::Il2CppString,
        is_hero_female: bool,
    ) -> ();

    #[method(name = "WaitUntilVoiceEnd", args = 0)]
    pub fn wait_until_voice_end(self) -> ();

    #[method(name = "GetDesc", args = 0)]
    pub fn get_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "Create", args = 1)]
    pub fn create(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-titlesequence")]
impl TitleSequence_ProcTitleCall {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleSequence_ProcTitleCall),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleSequence_ProcTitleCallMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlesequence/TitleSequence_PedestalData.md")))]
#[::unity2::class(namespace = "App", name = "TitleSequence.PedestalData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: titlesequence :: TitleSequence_PedestalData >)]
pub struct TitleSequence_PedestalData {}

#[cfg(feature = "app-titlesequence")]
#[::unity2::methods]
impl TitleSequence_PedestalData {
    #[method(name = "get_PedestalName", args = 0)]
    pub fn get_pedestal_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PedestalName", args = 1)]
    pub fn set_pedestal_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();
}

#[cfg(feature = "app-titlesequence")]
impl TitleSequence_PedestalData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleSequence_PedestalData),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleSequence_PedestalDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlesequence/TitleSequence.md")))]
#[::unity2::class(namespace = "App", name = "TitleSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: titlesequence :: TitleSequence >)]
pub struct TitleSequence {
    #[static_field]
    #[rename(name = "MapSceneName")]
    pub map_scene_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "CameraPrefabPath")]
    pub camera_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FadeOutSec")]
    pub fade_out_sec: f32,
    #[static_field]
    #[rename(name = "StartFadeOutElapsedTime")]
    pub start_fade_out_elapsed_time: f32,
    #[static_field]
    #[rename(name = "s_TitleLogoTexturePath")]
    pub s_title_logo_texture_path: ::unity2::Il2CppString,
    #[rename(name = "m_ElapsedTime")]
    pub m_elapsed_time: f32,
    #[rename(name = "m_TitleLogoAnimator")]
    pub m_title_logo_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PressStartAnimator")]
    pub m_press_start_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_GameVersionText")]
    pub m_game_version_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-titlesequence")]
#[::unity2::methods]
impl TitleSequence {
    #[method(name = "PostBgmEvent", args = 1)]
    pub fn post_bgm_event(self, bgm_event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(self, is_bgm_fade_out: bool) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "GetDesc", args = 0)]
    pub fn get_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "LoadTitleScene", args = 1)]
    pub fn load_title_scene(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "IsLoadingTitleScene", args = 1)]
    pub fn is_loading_title_scene(super_: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "UnloadTitleScene", args = 1)]
    pub fn unload_title_scene(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "InitAfterLoadTitleScene", args = 0)]
    pub fn init_after_load_title_scene() -> ();

    #[method(name = "ShowTitleScene", args = 0)]
    pub fn show_title_scene() -> ();

    #[method(name = "HideTitleScene", args = 0)]
    pub fn hide_title_scene() -> ();

    #[method(name = "SetTitleSceneVisible", args = 2)]
    pub fn set_title_scene_visible(scene_name: ::unity2::Il2CppString, is_show: bool) -> ();

    #[method(name = "GetMovieCanvasPrefab", args = 0)]
    pub fn get_movie_canvas_prefab() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-titlesequence")]
impl TitleSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleSequenceMethods>::ctor(this);
        this
    }
}
