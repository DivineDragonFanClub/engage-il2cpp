
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptsound/ScriptSound.md")))]
#[::unity2::class(namespace = "App", name = "ScriptSound")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptSound {}

#[cfg(feature = "app-scriptsound")]
#[::unity2::methods]
impl ScriptSound {
    #[method(name = "SoundPostEvent", args = 1)]
    pub fn sound_post_event(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SetFieldPhaseBgm", args = 1)]
    pub fn set_field_phase_bgm(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SetFieldBgmVolume", args = 1)]
    pub fn set_field_bgm_volume(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "PlayFieldBgm", args = 1)]
    pub fn play_field_bgm(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "PauseFieldBgm", args = 1)]
    pub fn pause_field_bgm(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "ResumeFieldBgm", args = 1)]
    pub fn resume_field_bgm(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SetFieldBgmWarSituation", args = 1)]
    pub fn set_field_bgm_war_situation(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scriptsound")]
impl ScriptSound {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptSound),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptSoundMethods>::ctor(this);
        this
    }
}
