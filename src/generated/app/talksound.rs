
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talksound/TalkSound_WaitSE.md")))]
#[::unity2::class(namespace = "App", name = "TalkSound.WaitSE")]
#[parent(crate::app::procinst::ProcInst)]
pub struct TalkSound_WaitSE {
    #[static_field]
    #[rename(name = "MinimumWaitSec")]
    pub minimum_wait_sec: f32,
    #[rename(name = "m_Sec")]
    pub m_sec: f32,
}

#[cfg(feature = "app-talksound")]
#[::unity2::methods]
impl TalkSound_WaitSE {
    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talksound")]
impl TalkSound_WaitSE {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkSound_WaitSE),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkSound_WaitSEMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talksound/TalkSound.md")))]
#[::unity2::class(namespace = "App", name = "TalkSound")]
#[parent(crate::system::object::Object)]
pub struct TalkSound {
    #[rename(name = "m_Mid")]
    pub m_mid: ::unity2::Il2CppString,
    #[rename(name = "m_SoundCmdExecBefore")]
    pub m_sound_cmd_exec_before: ::unity2::Il2CppString,
    #[rename(name = "m_SoundCmdExecAfter")]
    pub m_sound_cmd_exec_after: ::unity2::Il2CppString,
    #[rename(name = "m_ReservedTalkVoice")]
    pub m_reserved_talk_voice: ::unity2::Il2CppString,
    #[rename(name = "m_ReservedPersonVoice")]
    pub m_reserved_person_voice: ::unity2::Il2CppString,
    #[rename(name = "m_ReservedPersonPid")]
    pub m_reserved_person_pid: ::unity2::Il2CppString,
    #[rename(name = "m_ReservedPersonSwitchName")]
    pub m_reserved_person_switch_name: ::unity2::Il2CppString,
    #[rename(name = "m_PersonVoice")]
    pub m_person_voice: ::unity2::Il2CppString,
}

#[cfg(feature = "app-talksound")]
#[::unity2::methods]
impl TalkSound {
    #[method(name = "GetPersonVoice", args = 0)]
    pub fn get_person_voice(self) -> ::unity2::Il2CppString;

    #[method(name = "Init", args = 1)]
    pub fn init(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "TickBefore", args = 1)]
    pub fn tick_before(self, parent_proc: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "TickAfter", args = 1)]
    pub fn tick_after(self, parent_proc: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "PostTalkStartSoundEvent", args = 0)]
    pub fn post_talk_start_sound_event(self) -> ();

    #[method(name = "PostTalkEndSoundEvent", args = 0)]
    pub fn post_talk_end_sound_event(self) -> ();

    #[method(name = "GetArgs", args = 1)]
    pub fn get_args(self, line: ::unity2::Il2CppString) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetSoundType", args = 1)]
    pub fn get_sound_type(
        self,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::talksound::TalkSound_SoundType;

    #[method(name = "ExecLine", args = 2)]
    pub fn exec_line(
        self,
        line: ::unity2::Il2CppString,
        parent_proc: crate::app::procinst::ProcInst,
    ) -> bool;

    #[method(name = "GetLine", args = 2)]
    pub fn get_line(
        self,
        str: ::unity2::Il2CppString,
        line: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "TryPlayReservedVoice", args = 1)]
    pub fn try_play_reserved_voice(self, character: crate::combat::character::Character) -> ();

    #[method(name = "TryPlayReservedTalkVoice", args = 1)]
    pub fn try_play_reserved_talk_voice(
        self,
        character: crate::combat::character::Character,
    ) -> bool;

    #[method(name = "TryPlayReservedPersonVoice", args = 1)]
    pub fn try_play_reserved_person_voice(
        self,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "StopAllVoice", args = 0)]
    pub fn stop_all_voice(self) -> ();

    #[method(name = "OnDrawMonitor", args = 1)]
    pub fn on_draw_monitor(self, monitor: crate::app::debugmonitor::DebugMonitor) -> ();

    #[method(name = "MessFileNameToSoundBankName", args = 1)]
    pub fn mess_file_name_to_sound_bank_name(
        mess_file_name: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "GetRelianceSoundBankName", args = 1)]
    pub fn get_reliance_sound_bank_name(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetRelianceSoundBankNameAll", args = 0)]
    pub fn get_reliance_sound_bank_name_all(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "GetGodRelianceSoundBankName", args = 1)]
    pub fn get_god_reliance_sound_bank_name(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetGodRelianceSoundBankNameAll", args = 0)]
    pub fn get_god_reliance_sound_bank_name_all(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "GetHubCommonSoundBankName", args = 1)]
    pub fn get_hub_common_sound_bank_name(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetHubCommonSoundBankNameAll", args = 0)]
    pub fn get_hub_common_sound_bank_name_all(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "MidToSoundEvent", args = 1)]
    pub fn mid_to_sound_event(mid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int(str: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(str: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetFadeSpeedMsec", args = 1)]
    pub fn get_fade_speed_msec(str: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetForceType", args = 1)]
    pub fn get_force_type(str: ::unity2::Il2CppString) -> crate::app::force::Force_Type;

    #[method(name = "TryExecAction", args = 1)]
    pub fn try_exec_action(self, args: ::unity2::Array<::unity2::Il2CppString>) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talksound")]
impl TalkSound {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkSound),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkSoundMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talksound/TalkSound_SoundType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TalkSound_SoundType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TalkSound_SoundType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TalkSound.SoundType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TalkSound_SoundType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TalkSound_SoundType {
    pub fn se() -> Self {
        Self { value: 0 }
    }

    pub fn voice() -> Self {
        Self { value: 1 }
    }

    pub fn general() -> Self {
        Self { value: 2 }
    }

    pub fn bgm() -> Self {
        Self { value: 2 }
    }

    pub fn env() -> Self {
        Self { value: 2 }
    }
}
