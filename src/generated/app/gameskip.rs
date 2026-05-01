
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSkip_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSkip_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSkip.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSkip_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSkip_Status {
    pub fn disable() -> Self {
        Self { value: 1 }
    }

    pub fn trigger() -> Self {
        Self { value: 2 }
    }

    pub fn escape() -> Self {
        Self { value: 4 }
    }

    pub fn short_skipable() -> Self {
        Self { value: 8 }
    }

    pub fn short_skipping() -> Self {
        Self { value: 16 }
    }

    pub fn trigger_ai() -> Self {
        Self { value: 32 }
    }

    pub fn disable_ai_skip() -> Self {
        Self { value: 64 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSkip_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSkip_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSkip.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSkip_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSkip_Result {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn short_skip() -> Self {
        Self { value: 1 }
    }

    pub fn long_skip() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip.md")))]
#[::unity2::class(namespace = "App", name = "GameSkip")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: gameskip :: GameSkip >)]
pub struct GameSkip {
    #[rename(name = "m_Sequence")]
    pub m_sequence: crate::app::gameskip::GameSkip_Sequence,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::gameskip::GameSkip_Status,
    #[rename(name = "m_Statck")]
    pub m_statck: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::gameskip::GameSkip_Status,
    >,
}

#[cfg(feature = "app-gameskip")]
#[::unity2::methods]
impl GameSkip {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CalcSkipInput", args = 0)]
    pub fn calc_skip_input(self) -> crate::app::gameskip::GameSkip_Result;

    #[method(name = "TstStatus", args = 1)]
    pub fn tst_status(self, status: crate::app::gameskip::GameSkip_Status) -> bool;

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(self, status: crate::app::gameskip::GameSkip_Status) -> ();

    #[method(name = "ClearStatus", args = 1)]
    pub fn clear_status(self, status: crate::app::gameskip::GameSkip_Status) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "DebugSkip", args = 0)]
    pub fn debug_skip() -> bool;

    #[method(name = "Trigger", args = 0)]
    pub fn trigger() -> ();

    #[method(name = "Escape", args = 0)]
    pub fn escape() -> ();

    #[method(name = "Enable", args = 0)]
    pub fn enable() -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable() -> ();

    #[method(name = "EnableShortSkip", args = 0)]
    pub fn enable_short_skip() -> ();

    #[method(name = "DisableShortSkip", args = 0)]
    pub fn disable_short_skip() -> ();

    #[method(name = "IsSkip", args = 0)]
    pub fn is_skip() -> bool;

    #[method(name = "IsDisable", args = 0)]
    pub fn is_disable() -> bool;

    #[method(name = "IsWait", args = 0)]
    pub fn is_wait() -> bool;

    #[method(name = "IsBlackOut", args = 0)]
    pub fn is_black_out() -> bool;

    #[method(name = "IsBlackOrFadeOut", args = 0)]
    pub fn is_black_or_fade_out() -> bool;

    #[method(name = "IsFadeIn", args = 0)]
    pub fn is_fade_in() -> bool;

    #[method(name = "PushState", args = 0)]
    pub fn push_state() -> ();

    #[method(name = "PopState", args = 0)]
    pub fn pop_state() -> ();

    #[method(name = "PushDisableAISkip", args = 0)]
    pub fn push_disable_ai_skip() -> ();

    #[method(name = "PopDisableAISkip", args = 0)]
    pub fn pop_disable_ai_skip() -> ();

    #[method(name = "TriggerAI", args = 0)]
    pub fn trigger_ai() -> ();

    #[method(name = "WaitTime", args = 2)]
    pub fn wait_time(super_: crate::app::procinst::ProcInst, time: f32) -> ();

    #[method(name = "PWaitTime", args = 1)]
    pub fn p_wait_time(time: f32) -> crate::app::procdesc::ProcDesc;

    #[method(name = "SuspendBind", args = 1)]
    pub fn suspend_bind(super_: crate::app::procinst::ProcInst) -> crate::app::procinst::ProcInst;
}

#[cfg(feature = "app-gameskip")]
impl GameSkip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSkip),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSkipMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip_ProcSuspend.md")))]
#[::unity2::class(namespace = "App", name = "GameSkip.ProcSuspend")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GameSkip_ProcSuspend {}

#[cfg(feature = "app-gameskip")]
#[::unity2::methods]
impl GameSkip_ProcSuspend {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameskip")]
impl GameSkip_ProcSuspend {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSkip_ProcSuspend),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSkip_ProcSuspendMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip_Sequence.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSkip_Sequence {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSkip_Sequence {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSkip.Sequence";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSkip_Sequence {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSkip_Sequence {
    pub fn tick() -> Self {
        Self { value: 0 }
    }

    pub fn fade_out() -> Self {
        Self { value: 1 }
    }

    pub fn executed() -> Self {
        Self { value: 2 }
    }

    pub fn fade_in() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameskip/GameSkip_ProcWaitTime.md")))]
#[::unity2::class(namespace = "App", name = "GameSkip.ProcWaitTime")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GameSkip_ProcWaitTime {
    #[rename(name = "m_WaitTime")]
    pub m_wait_time: f32,
    #[rename(name = "m_DeltaTime")]
    pub m_delta_time: f32,
}

#[cfg(feature = "app-gameskip")]
#[::unity2::methods]
impl GameSkip_ProcWaitTime {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, time: f32) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();
}

#[cfg(feature = "app-gameskip")]
impl GameSkip_ProcWaitTime {
    pub fn new(time: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSkip_ProcWaitTime),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSkip_ProcWaitTimeMethods>::ctor(this, time);
        this
    }
}
