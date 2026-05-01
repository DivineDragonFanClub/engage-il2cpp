
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/floattween/FloatTween.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FloatTween {
    pub m_target:
        crate::unity_engine::ui::coroutine_tween::floattween::FloatTween_FloatTweenCallback,
    pub m_start_value: f32,
    pub m_target_value: f32,
    pub m_duration: f32,
    pub m_ignore_time_scale: bool,
}

impl ::unity2::ClassIdentity for FloatTween {
    const NAMESPACE: &'static str = "UnityEngine.UI.CoroutineTween";

    const NAME: &'static str = "FloatTween";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FloatTween {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-floattween")]
#[::unity2::methods(value)]
impl FloatTween {
    #[method(name = "get_startValue", args = 0)]
    pub fn get_start_value(self) -> f32;

    #[method(name = "set_startValue", args = 1)]
    pub fn set_start_value(self, value: f32) -> ();

    #[method(name = "get_targetValue", args = 0)]
    pub fn get_target_value(self) -> f32;

    #[method(name = "set_targetValue", args = 1)]
    pub fn set_target_value(self, value: f32) -> ();

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
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<f32>,
    ) -> ();

    #[method(name = "GetIgnoreTimescale", args = 0)]
    pub fn get_ignore_timescale(self) -> bool;

    #[method(name = "ValidTarget", args = 0)]
    pub fn valid_target(self) -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/floattween/FloatTween_FloatTweenCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI.CoroutineTween",
    name = "FloatTween.FloatTweenCallback"
)]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < f32 >)]
pub struct FloatTween_FloatTweenCallback {}

#[cfg(feature = "unity_engine-ui-coroutine_tween-floattween")]
#[::unity2::methods]
impl FloatTween_FloatTweenCallback {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-floattween")]
impl FloatTween_FloatTweenCallback {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FloatTween_FloatTweenCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IFloatTween_FloatTweenCallbackMethods>::ctor(this);
        this
    }
}
