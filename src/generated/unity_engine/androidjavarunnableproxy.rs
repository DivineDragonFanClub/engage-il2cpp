
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::androidjavaproxy::AndroidJavaProxy;
use crate::unity_engine::androidjavaproxy::IAndroidJavaProxy;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjavarunnableproxy/AndroidJavaRunnableProxy.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJavaRunnableProxy")]
#[parent(crate::unity_engine::androidjavaproxy::AndroidJavaProxy)]
pub struct AndroidJavaRunnableProxy {
    #[rename(name = "mRunnable")]
    pub m_runnable: crate::unity_engine::androidjavarunnable::AndroidJavaRunnable,
}

#[cfg(feature = "unity_engine-androidjavarunnableproxy")]
#[::unity2::methods]
impl AndroidJavaRunnableProxy {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        runnable: crate::unity_engine::androidjavarunnable::AndroidJavaRunnable,
    ) -> ();
}

#[cfg(feature = "unity_engine-androidjavarunnableproxy")]
impl AndroidJavaRunnableProxy {
    pub fn new(runnable: crate::unity_engine::androidjavarunnable::AndroidJavaRunnable) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaRunnableProxy),
                ::core::stringify!(new),
            )
        });
        <Self as IAndroidJavaRunnableProxyMethods>::ctor(this, runnable);
        this
    }
}
