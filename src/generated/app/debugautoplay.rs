
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugautoplay/DebugAutoPlay.md")))]
#[::unity2::class(namespace = "App", name = "DebugAutoPlay")]
#[parent(crate::system::object::Object)]
pub struct DebugAutoPlay {}

#[cfg(feature = "app-debugautoplay")]
#[::unity2::methods]
impl DebugAutoPlay {
    #[method(name = "SetEnable", args = 1)]
    pub fn set_enable(enable: bool) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable() -> bool;

    #[method(name = "IsSkip", args = 0)]
    pub fn is_skip() -> bool;

    #[method(name = "IsMonitor", args = 0)]
    pub fn is_monitor() -> bool;

    #[method(name = "Update", args = 0)]
    pub fn update() -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name() -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugautoplay")]
impl DebugAutoPlay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugAutoPlay),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugAutoPlayMethods>::ctor(this);
        this
    }
}
