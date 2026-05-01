
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::marker::IMarker;
use crate::unity_engine::timeline::marker::Marker;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonriderestartwindmarker/DragonRideRestartWindMarker.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideRestartWindMarker")]
#[parent(crate::unity_engine::timeline::marker::Marker)]
pub struct DragonRideRestartWindMarker {}

#[cfg(feature = "app-dragonriderestartwindmarker")]
#[::unity2::methods]
impl DragonRideRestartWindMarker {
    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> crate::unity_engine::propertyname::PropertyName;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonriderestartwindmarker")]
impl DragonRideRestartWindMarker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideRestartWindMarker),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideRestartWindMarkerMethods>::ctor(this);
        this
    }
}
