
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/coroutine_tween/tweenrunner_1/TweenRunner_1.md")))]
#[::unity2::class(namespace = "UnityEngine.UI.CoroutineTween", name = "TweenRunner`1")]
pub struct TweenRunner_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_CoroutineContainer")]
    pub m_coroutine_container: crate::unity_engine::monobehaviour::MonoBehaviour,
    #[rename(name = "m_Tween")]
    pub m_tween: crate::system::collections::ienumerator::IEnumerator,
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-tweenrunner_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> TweenRunner_1<T0> {
    #[method(name = "Start", args = 1)]
    pub fn start(tween_info: T0) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Init", args = 1)]
    pub fn init(self, coroutine_container: crate::unity_engine::monobehaviour::MonoBehaviour)
        -> ();

    #[method(name = "StartTween", args = 1)]
    pub fn start_tween(self, info: T0) -> ();

    #[method(name = "StopTween", args = 0)]
    pub fn stop_tween(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-coroutine_tween-tweenrunner_1")]
impl<T0: ::unity2::ClassIdentity> TweenRunner_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TweenRunner_1),
                ::core::stringify!(new),
            )
        });
        <Self as ITweenRunner_1Methods<T0>>::ctor(this);
        this
    }
}
