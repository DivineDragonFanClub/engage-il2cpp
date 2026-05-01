
use crate::app::singletonscriptableobject_1::ISingletonScriptableObject_1;
use crate::app::singletonscriptableobject_1::SingletonScriptableObject_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debuginfo/DebugInfo.md")))]
#[::unity2::class(namespace = "App", name = "DebugInfo")]
# [parent (crate :: app :: singletonscriptableobject_1 :: SingletonScriptableObject_1 < crate :: app :: debuginfo :: DebugInfo >)]
pub struct DebugInfo {
    #[rename(name = "SkipWarnings")]
    pub skip_warnings: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-debuginfo")]
#[::unity2::methods]
impl DebugInfo {
    #[method(name = "IsSkip", args = 1)]
    pub fn is_skip(log_text: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debuginfo")]
impl DebugInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugInfoMethods>::ctor(this);
        this
    }
}
