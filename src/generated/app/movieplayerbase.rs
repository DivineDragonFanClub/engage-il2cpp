
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/movieplayerbase/MoviePlayerBase_Caption.md")))]
#[::unity2::class(namespace = "App", name = "MoviePlayerBase.Caption")]
#[parent(crate::system::object::Object)]
pub struct MoviePlayerBase_Caption {
    #[rename(name = "m_Text")]
    pub m_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
    #[rename(name = "m_IsShow")]
    pub m_is_show: bool,
    #[rename(name = "m_IsShowOld")]
    pub m_is_show_old: bool,
}

#[cfg(feature = "app-movieplayerbase")]
#[::unity2::methods]
impl MoviePlayerBase_Caption {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: crate::tm_pro::textmeshprougui::TextMeshProUGUI) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "SetText", args = 2)]
    pub fn set_text(self, mid: ::unity2::Il2CppString, msec: i32) -> ();

    #[method(name = "ClearText", args = 0)]
    pub fn clear_text(self) -> ();

    #[method(name = "IsShow", args = 0)]
    pub fn is_show(self) -> bool;

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SwitchShow", args = 0)]
    pub fn switch_show(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-movieplayerbase")]
impl MoviePlayerBase_Caption {
    pub fn new(text: crate::tm_pro::textmeshprougui::TextMeshProUGUI) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoviePlayerBase_Caption),
                ::core::stringify!(new),
            )
        });
        <Self as IMoviePlayerBase_CaptionMethods>::ctor(this, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/movieplayerbase/MoviePlayerBase.md")))]
#[::unity2::class(namespace = "App", name = "MoviePlayerBase")]
#[parent(crate::system::object::Object)]
pub struct MoviePlayerBase {
    #[static_field]
    #[rename(name = "UseFileExtName")]
    pub use_file_ext_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UseSceneName")]
    pub use_scene_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HeroFemalePostfix")]
    pub hero_female_postfix: ::unity2::Il2CppString,
    #[rename(name = "m_ScreenObject")]
    pub m_screen_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CanvasObject")]
    pub m_canvas_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Caption")]
    pub m_caption: crate::app::movieplayerbase::MoviePlayerBase_Caption,
    #[rename(name = "m_TitleLogo")]
    pub m_title_logo: crate::app::movieplayerbase::MoviePlayerBase_TitleLogo,
    #[rename(name = "m_KeyHelp")]
    pub m_key_help: crate::app::movieplayerbase::MoviePlayerBase_KeyHelp,
    #[rename(name = "m_IsMovieFileNameDirect")]
    pub m_is_movie_file_name_direct: bool,
    #[rename(name = "m_BaseMovieFileName")]
    pub m_base_movie_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_MovieFileName")]
    pub m_movie_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_SoundBankName")]
    pub m_sound_bank_name: ::unity2::Il2CppString,
    #[rename(name = "m_PrevPlayTime")]
    pub m_prev_play_time: f64,
    #[rename(name = "m_IsErrorOccured")]
    pub m_is_error_occured: bool,
    #[rename(name = "m_SoundEventOnStartViewer1")]
    pub m_sound_event_on_start_viewer1: ::unity2::Il2CppString,
    #[rename(name = "m_SoundEventOnStartViewer2")]
    pub m_sound_event_on_start_viewer2: ::unity2::Il2CppString,
    #[rename(name = "m_SoundEventOnStartViewer3")]
    pub m_sound_event_on_start_viewer3: ::unity2::Il2CppString,
}

#[cfg(feature = "app-movieplayerbase")]
#[::unity2::methods]
impl MoviePlayerBase {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "Load", args = 2)]
    pub fn load(
        self,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "ResolveMovieFileName", args = 3)]
    pub fn resolve_movie_file_name(
        base_movie_file_name: ::unity2::Il2CppString,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "GetMovieFilePath", args = 0)]
    pub fn get_movie_file_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMovieFilePath", args = 2)]
    pub fn get_movie_file_path_2(
        base_movie_file_name: ::unity2::Il2CppString,
        movie_file_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LoadSceneBind", args = 1)]
    pub fn load_scene_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "UnloadSceneBind", args = 1)]
    pub fn unload_scene_bind(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "SetupAfterLoadScene", args = 0)]
    pub fn setup_after_load_scene(self) -> bool;

    #[method(name = "SetupByMovieCanvasPrefab", args = 1)]
    pub fn setup_by_movie_canvas_prefab(
        self,
        canvas_prefab: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetupCommon", args = 0)]
    pub fn setup_common(self) -> bool;

    #[method(name = "SetMovieFilePathToVideoPlayer", args = 0)]
    pub fn set_movie_file_path_to_video_player(self) -> bool;

    #[method(name = "EnableScene", args = 0)]
    pub fn enable_scene(self) -> ();

    #[method(name = "DisableScene", args = 0)]
    pub fn disable_scene(self) -> ();

    #[method(name = "SetSceneActive", args = 1)]
    pub fn set_scene_active(self, is_active: bool) -> ();

    #[method(name = "EnableCanvas", args = 0)]
    pub fn enable_canvas(self) -> ();

    #[method(name = "DisableCanvas", args = 0)]
    pub fn disable_canvas(self) -> ();

    #[method(name = "ShowHelp", args = 0)]
    pub fn show_help(self) -> ();

    #[method(name = "HideHelp", args = 0)]
    pub fn hide_help(self) -> ();

    #[method(name = "IsShowCaption", args = 0)]
    pub fn is_show_caption(self) -> bool;

    #[method(name = "SetCaption", args = 2)]
    pub fn set_caption(self, mid: ::unity2::Il2CppString, msec: i32) -> ();

    #[method(name = "SwitchCaptionOnOff", args = 0)]
    pub fn switch_caption_on_off(self) -> ();

    #[method(name = "ShowTitleLogo", args = 1)]
    pub fn show_title_logo(self, msec: i32) -> ();

    #[method(name = "PostSoundEventOnStartViewer", args = 0)]
    pub fn post_sound_event_on_start_viewer(self) -> ();

    #[method(name = "PostSoundEventOnEnd", args = 0)]
    pub fn post_sound_event_on_end(self) -> ();

    #[method(name = "PostSoundEventOnEnd", args = 1)]
    pub fn post_sound_event_on_end_2(movie_file_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PostSoundEvent", args = 1)]
    pub fn post_sound_event(sound_event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetSoundEventNameOnStart", args = 1)]
    pub fn get_sound_event_name_on_start(
        movie_file_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetScreenObjectName", args = 0)]
    pub fn get_screen_object_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsPaused", args = 0)]
    pub fn is_paused(self) -> bool;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "IsPlayEnd", args = 0)]
    pub fn is_play_end(self) -> bool;

    #[method(name = "IsErrorOccured", args = 0)]
    pub fn is_error_occured(self) -> bool;

    #[method(name = "GetMovieLength", args = 0)]
    pub fn get_movie_length(self) -> f32;

    #[method(name = "GetPlayingPosition", args = 0)]
    pub fn get_playing_position(self) -> f32;

    #[method(name = "Persistent", args = 0)]
    pub fn persistent(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "IsPrepared", args = 0)]
    pub fn is_prepared(self) -> bool;

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "SuspendOn", args = 0)]
    pub fn suspend_on(self) -> ();

    #[method(name = "SuspendOff", args = 0)]
    pub fn suspend_off(self) -> ();
}

#[cfg(feature = "app-movieplayerbase")]
impl MoviePlayerBase {
    pub fn new(movie_file_name: ::unity2::Il2CppString, is_movie_file_name_direct: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoviePlayerBase),
                ::core::stringify!(new),
            )
        });
        <Self as IMoviePlayerBaseMethods>::ctor(this, movie_file_name, is_movie_file_name_direct);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/movieplayerbase/MoviePlayerBase_KeyHelp.md")))]
#[::unity2::class(namespace = "App", name = "MoviePlayerBase.KeyHelp")]
#[parent(crate::system::object::Object)]
pub struct MoviePlayerBase_KeyHelp {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KeyHelpController")]
    pub m_key_help_controller: crate::app::keyhelpcontroller::KeyHelpController,
    #[rename(name = "m_IsShow")]
    pub m_is_show: bool,
    #[rename(name = "m_DispTimer")]
    pub m_disp_timer: f32,
}

#[cfg(feature = "app-movieplayerbase")]
#[::unity2::methods]
impl MoviePlayerBase_KeyHelp {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-movieplayerbase")]
impl MoviePlayerBase_KeyHelp {
    pub fn new(root_object: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoviePlayerBase_KeyHelp),
                ::core::stringify!(new),
            )
        });
        <Self as IMoviePlayerBase_KeyHelpMethods>::ctor(this, root_object);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/movieplayerbase/MoviePlayerBase_TitleLogo.md")))]
#[::unity2::class(namespace = "App", name = "MoviePlayerBase.TitleLogo")]
#[parent(crate::system::object::Object)]
pub struct MoviePlayerBase_TitleLogo {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Duration")]
    pub m_duration: f32,
    #[rename(name = "m_TextureResourceHandle")]
    pub m_texture_resource_handle:
        crate::app::tresourcehandle_1::TResourceHandle_1<crate::unity_engine::sprite::Sprite>,
}

#[cfg(feature = "app-movieplayerbase")]
#[::unity2::methods]
impl MoviePlayerBase_TitleLogo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        image: crate::unity_engine::ui::image::Image,
        animator: crate::unity_engine::animator::Animator,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Show", args = 1)]
    pub fn show(self, msec: i32) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-movieplayerbase")]
impl MoviePlayerBase_TitleLogo {
    pub fn new(
        image: crate::unity_engine::ui::image::Image,
        animator: crate::unity_engine::animator::Animator,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoviePlayerBase_TitleLogo),
                ::core::stringify!(new),
            )
        });
        <Self as IMoviePlayerBase_TitleLogoMethods>::ctor(this, image, animator);
        this
    }
}
