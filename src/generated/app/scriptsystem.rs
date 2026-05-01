
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptsystem/ScriptSystem_CameraList.md")))]
#[::unity2::class(namespace = "App", name = "ScriptSystem.CameraList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: camera :: Camera >)]
pub struct ScriptSystem_CameraList {}

#[cfg(feature = "app-scriptsystem")]
#[::unity2::methods]
impl ScriptSystem_CameraList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, tag: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "Find", args = 1)]
    pub fn find(self, name: ::unity2::Il2CppString) -> crate::unity_engine::camera::Camera;
}

#[cfg(feature = "app-scriptsystem")]
impl ScriptSystem_CameraList {
    pub fn new(tag: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptSystem_CameraList),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptSystem_CameraListMethods>::ctor(this, tag);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptsystem/ScriptSystem.md")))]
#[::unity2::class(namespace = "App", name = "ScriptSystem")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptSystem {}

#[cfg(feature = "app-scriptsystem")]
#[::unity2::methods]
impl ScriptSystem {
    #[method(name = "Include", args = 1)]
    pub fn include(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>)
        -> ();

    #[method(name = "YieldImpl", args = 1)]
    pub fn yield_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "JumpImpl", args = 1)]
    pub fn jump_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Log", args = 1)]
    pub fn log(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "Warning", args = 1)]
    pub fn warning(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>)
        -> ();

    #[method(name = "TimeGetDelta", args = 1)]
    pub fn time_get_delta(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SkipIsBlackOut", args = 1)]
    pub fn skip_is_black_out(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SkipEscape", args = 1)]
    pub fn skip_escape(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SkipPushStateAndDisable", args = 1)]
    pub fn skip_push_state_and_disable(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SkipPopState", args = 1)]
    pub fn skip_pop_state(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Talk", args = 1)]
    pub fn talk(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "TalkBeginContinue", args = 1)]
    pub fn talk_begin_continue(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TalkEndContinue", args = 1)]
    pub fn talk_end_continue(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Dialog", args = 1)]
    pub fn dialog(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "Notice", args = 1)]
    pub fn notice(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "MessLoad", args = 1)]
    pub fn mess_load(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MessFree", args = 1)]
    pub fn mess_free(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MessIsExist", args = 1)]
    pub fn mess_is_exist(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MessSetArgumentImpl", args = 2)]
    pub fn mess_set_argument_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        offset: i32,
    ) -> ();

    #[method(name = "MessSetArgumentImpl", args = 2)]
    pub fn mess_set_argument_impl_2(
        index: i32,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "MessSetArgument", args = 1)]
    pub fn mess_set_argument(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SceneGetName", args = 1)]
    pub fn scene_get_name(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SceneLoad", args = 1)]
    pub fn scene_load(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "SceneUnload", args = 1)]
    pub fn scene_unload(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "PuppetDemo", args = 1)]
    pub fn puppet_demo(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CutScene", args = 1)]
    pub fn cut_scene(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Movie", args = 1)]
    pub fn movie(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "Telop", args = 1)]
    pub fn telop(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "ModeSelect", args = 1)]
    pub fn mode_select(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "WinRule", args = 1)]
    pub fn win_rule(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Tutorial", args = 1)]
    pub fn tutorial(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CameraGetMain", args = 1)]
    pub fn camera_get_main(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetEventCamera", args = 1)]
    pub fn get_event_camera(
        camera: crate::unity_engine::camera::Camera,
    ) -> crate::app::eventcamera::EventCamera;

    #[method(name = "CameraSetMain", args = 1)]
    pub fn camera_set_main(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "VariableEntry", args = 1)]
    pub fn variable_entry(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "VariableIsExist", args = 1)]
    pub fn variable_is_exist(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "VariableSet", args = 1)]
    pub fn variable_set(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "VariableGet", args = 1)]
    pub fn variable_get(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "FadeIn", args = 1)]
    pub fn fade_in(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>)
        -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "IsFading", args = 1)]
    pub fn is_fading(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsLoading", args = 1)]
    pub fn is_loading(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "VariableEntryMonitor", args = 1)]
    pub fn variable_entry_monitor(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "VariableClearMonitor", args = 1)]
    pub fn variable_clear_monitor(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GetDebugButtons", args = 2)]
    pub fn get_debug_buttons(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::nn::hid::npadbutton::NpadButton;

    #[method(name = "DebugIsButton", args = 1)]
    pub fn debug_is_button(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DebugIsTrigger", args = 1)]
    pub fn debug_is_trigger(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DebugHookButton", args = 1)]
    pub fn debug_hook_button(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugHookStickL", args = 1)]
    pub fn debug_hook_stick_l(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugHookStickR", args = 1)]
    pub fn debug_hook_stick_r(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugCreateMenu", args = 1)]
    pub fn debug_create_menu(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugSetParam", args = 1)]
    pub fn debug_set_param(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugGetParam", args = 1)]
    pub fn debug_get_param(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DebugIsProcExists", args = 1)]
    pub fn debug_is_proc_exists(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DebugIsAutoPlay", args = 1)]
    pub fn debug_is_auto_play(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ObjectIsExist", args = 1)]
    pub fn object_is_exist(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ObjectCreate", args = 1)]
    pub fn object_create(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "ObjectDelete", args = 1)]
    pub fn object_delete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "ObjectSetActive", args = 1)]
    pub fn object_set_active(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "ObjectActivate", args = 1)]
    pub fn object_activate(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "ObjectDeactivate", args = 1)]
    pub fn object_deactivate(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HasContent", args = 1)]
    pub fn has_content(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DebugGpuPerf", args = 1)]
    pub fn debug_gpu_perf(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "DebugAddCoroutine", args = 1)]
    pub fn debug_add_coroutine(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "RandomGet", args = 1)]
    pub fn random_get(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SubPrefix", args = 1)]
    pub fn sub_prefix(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "StringContains", args = 1)]
    pub fn string_contains(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetMainCamera", args = 0)]
    pub fn get_main_camera() -> crate::unity_engine::camera::Camera;

    #[method(name = "BackgroundColorSet", args = 1)]
    pub fn background_color_set(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scriptsystem")]
impl ScriptSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptSystemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptsystem/ScriptSystem_DebugButton.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ScriptSystem_DebugButton {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ScriptSystem_DebugButton {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ScriptSystem.DebugButton";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptSystem_DebugButton {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ScriptSystem_DebugButton {
    pub fn a() -> Self {
        Self { value: 1 }
    }

    pub fn b() -> Self {
        Self { value: 2 }
    }

    pub fn x() -> Self {
        Self { value: 4 }
    }

    pub fn y() -> Self {
        Self { value: 8 }
    }

    pub fn l() -> Self {
        Self { value: 16 }
    }

    pub fn r() -> Self {
        Self { value: 32 }
    }

    pub fn left() -> Self {
        Self { value: 64 }
    }

    pub fn up() -> Self {
        Self { value: 128 }
    }

    pub fn right() -> Self {
        Self { value: 256 }
    }

    pub fn down() -> Self {
        Self { value: 512 }
    }

    pub fn plus() -> Self {
        Self { value: 1024 }
    }

    pub fn minus() -> Self {
        Self { value: 2048 }
    }
}
