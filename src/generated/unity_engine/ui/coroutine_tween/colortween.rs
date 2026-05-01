
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/colortween/ColorTween_ColorTweenCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI.CoroutineTween",
    name = "ColorTween.ColorTweenCallback"
)]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < crate :: unity_engine :: color :: Color >)]
pub struct ColorTween_ColorTweenCallback {}

#[cfg(feature = "unity_engine-ui-coroutine_tween-colortween")]
#[::unity2::methods]
impl ColorTween_ColorTweenCallback {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-colortween")]
impl ColorTween_ColorTweenCallback {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ColorTween_ColorTweenCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IColorTween_ColorTweenCallbackMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/colortween/ColorTween_ColorTweenMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ColorTween_ColorTweenMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ColorTween_ColorTweenMode {
    const NAMESPACE: &'static str = "UnityEngine.UI.CoroutineTween";

    const NAME: &'static str = "ColorTween.ColorTweenMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ColorTween_ColorTweenMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ColorTween_ColorTweenMode {
    pub fn all() -> Self {
        Self { value: 0 }
    }

    pub fn rgb() -> Self {
        Self { value: 1 }
    }

    pub fn alpha() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/colortween/ColorTween.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ColorTween {
    pub m_target:
        crate::unity_engine::ui::coroutine_tween::colortween::ColorTween_ColorTweenCallback,
    pub m_start_color: crate::unity_engine::color::Color,
    pub m_target_color: crate::unity_engine::color::Color,
    pub m_tween_mode:
        crate::unity_engine::ui::coroutine_tween::colortween::ColorTween_ColorTweenMode,
    pub m_duration: f32,
    pub m_ignore_time_scale: bool,
}

impl ::unity2::ClassIdentity for ColorTween {
    const NAMESPACE: &'static str = "UnityEngine.UI.CoroutineTween";

    const NAME: &'static str = "ColorTween";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ColorTween {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-colortween")]
#[::unity2::methods(value)]
impl ColorTween {
    #[method(name = "get_startColor", args = 0)]
    pub fn get_start_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_startColor", args = 1)]
    pub fn set_start_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_targetColor", args = 0)]
    pub fn get_target_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_targetColor", args = 1)]
    pub fn set_target_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_tweenMode", args = 0)]
    pub fn get_tween_mode(
        self,
    ) -> crate::unity_engine::ui::coroutine_tween::colortween::ColorTween_ColorTweenMode;

    #[method(name = "set_tweenMode", args = 1)]
    pub fn set_tween_mode(
        self,
        value: crate::unity_engine::ui::coroutine_tween::colortween::ColorTween_ColorTweenMode,
    ) -> ();

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f32;

    #[method(name = "set_duration", args = 1)]
    pub fn set_duration(self, value: f32) -> ();

    #[method(name = "get_ignoreTimeScale", args = 0)]
    pub fn get_ignore_time_scale(self) -> bool;

    #[method(name = "set_ignoreTimeScale", args = 1)]
    pub fn set_ignore_time_scale(self, value: bool) -> ();

    #[method(name = "TweenValue", args = 1)]
    pub fn tween_value(self, float_percentage: f32) -> ();

    #[method(name = "AddOnChangedCallback", args = 1)]
    pub fn add_on_changed_callback(
        self,
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "GetIgnoreTimescale", args = 0)]
    pub fn get_ignore_timescale(self) -> bool;

    #[method(name = "ValidTarget", args = 0)]
    pub fn valid_target(self) -> bool;
}
