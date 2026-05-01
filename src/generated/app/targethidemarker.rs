
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::marker::IMarker;
use crate::unity_engine::timeline::marker::Marker;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/targethidemarker/TargetHideMarker.md")))]
#[::unity2::class(namespace = "App", name = "TargetHideMarker")]
#[parent(crate::unity_engine::timeline::marker::Marker)]
pub struct TargetHideMarker {
    #[rename(name = "targetLocator")]
    pub target_locator: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-targethidemarker")]
#[::unity2::methods]
impl TargetHideMarker {
    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> crate::unity_engine::propertyname::PropertyName;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-targethidemarker")]
impl TargetHideMarker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TargetHideMarker),
                ::core::stringify!(new),
            )
        });
        <Self as ITargetHideMarkerMethods>::ctor(this);
        this
    }
}
