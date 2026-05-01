
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/moviesequence/MovieSequence.md")))]
#[::unity2::class(namespace = "App", name = "MovieSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: moviesequence :: MovieSequence >)]
pub struct MovieSequence {
    #[static_field]
    #[rename(name = "PrepareRetryCountLimit")]
    pub prepare_retry_count_limit: i32,
    #[rename(name = "m_MoviePlayer")]
    pub m_movie_player: crate::app::movieplayer::MoviePlayer,
    #[rename(name = "m_IsFadeOutInStart")]
    pub m_is_fade_out_in_start: bool,
    #[rename(name = "m_IsWaitForPlayGOP")]
    pub m_is_wait_for_play_gop: bool,
    #[rename(name = "m_IsPlayGOPFromTitle")]
    pub m_is_play_gop_from_title: bool,
    #[rename(name = "m_PrepareRetryCount")]
    pub m_prepare_retry_count: i32,
}

#[cfg(feature = "app-moviesequence")]
#[::unity2::methods]
impl MovieSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "PushFade", args = 0)]
    pub fn push_fade(self) -> ();

    #[method(name = "PopFade", args = 0)]
    pub fn pop_fade(self) -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "SetupAfterLoadScene", args = 0)]
    pub fn setup_after_load_scene(self) -> ();

    #[method(name = "UnloadScene", args = 0)]
    pub fn unload_scene(self) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "WaitPrepare", args = 0)]
    pub fn wait_prepare(self) -> ();

    #[method(name = "PlayMovie", args = 0)]
    pub fn play_movie(self) -> ();

    #[method(name = "StopMovie", args = 0)]
    pub fn stop_movie(self) -> ();

    #[method(name = "IsTruePlayEnd", args = 0)]
    pub fn is_true_play_end(self) -> bool;

    #[method(name = "IsShowCaption", args = 0)]
    pub fn is_show_caption(self) -> bool;

    #[method(name = "SetCaption", args = 2)]
    pub fn set_caption(self, mid: ::unity2::Il2CppString, msec: i32) -> ();

    #[method(name = "ShowTitleLogo", args = 1)]
    pub fn show_title_logo(self, msec: i32) -> ();

    #[method(name = "EnableControllerSupport", args = 0)]
    pub fn enable_controller_support(self) -> ();

    #[method(name = "DisableControllerSupport", args = 0)]
    pub fn disable_controller_support(self) -> ();

    #[method(name = "WaitPlayMovie", args = 0)]
    pub fn wait_play_movie(self) -> ();

    #[method(name = "EnableScene", args = 0)]
    pub fn enable_scene(self) -> ();

    #[method(name = "DisableScene", args = 0)]
    pub fn disable_scene(self) -> ();

    #[method(name = "Persistent", args = 0)]
    pub fn persistent(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "PostSoundEventOnEnd", args = 0)]
    pub fn post_sound_event_on_end(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "StartCaption", args = 2)]
    pub fn start_caption(mid: ::unity2::Il2CppString, msec: i32) -> ();

    #[method(name = "StartTitleLogo", args = 1)]
    pub fn start_title_logo(msec: i32) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "IsSkip", args = 0)]
    pub fn is_skip() -> bool;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "SetupByTitleMovieCanvasPrefab", args = 0)]
    pub fn setup_by_title_movie_canvas_prefab(self) -> ();

    #[method(name = "InitLoopGOP", args = 0)]
    pub fn init_loop_gop(self) -> ();

    #[method(name = "EnableCanvas", args = 0)]
    pub fn enable_canvas(self) -> ();

    #[method(name = "DisableCanvas", args = 0)]
    pub fn disable_canvas(self) -> ();

    #[method(name = "SwitchGenderGOP", args = 0)]
    pub fn switch_gender_gop(self) -> ();

    #[method(name = "InitWaitForPlayGOP", args = 0)]
    pub fn init_wait_for_play_gop(self) -> ();

    #[method(name = "WaitForPlayGOP", args = 0)]
    pub fn wait_for_play_gop(self) -> bool;

    #[method(name = "PlayMovieGOP", args = 0)]
    pub fn play_movie_gop(self) -> ();

    #[method(name = "TickGOP", args = 0)]
    pub fn tick_gop(self) -> ();

    #[method(name = "FadeOutGOP", args = 0)]
    pub fn fade_out_gop(self) -> ();

    #[method(name = "SetPlayGOPFromTitle", args = 1)]
    pub fn set_play_gop_from_title(self, is_from_title: bool) -> ();

    #[method(name = "WaitEndForPlayGOP", args = 0)]
    pub fn wait_end_for_play_gop(self) -> ();

    #[method(name = "CreateBindGOPForTitleLoop", args = 2)]
    pub fn create_bind_gop_for_title_loop(
        super_: crate::app::procinst::ProcInst,
        is_hero_female: bool,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-moviesequence")]
impl MovieSequence {
    pub fn new(movie_file_name: ::unity2::Il2CppString, is_movie_file_name_direct: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MovieSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMovieSequenceMethods>::ctor(this, movie_file_name, is_movie_file_name_direct);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/moviesequence/MovieSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MovieSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MovieSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MovieSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MovieSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MovieSequence_Label {
    pub fn r#loop() -> Self {
        Self { value: 0 }
    }

    pub fn prepare() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}
