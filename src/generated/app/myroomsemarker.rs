
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::marker::IMarker;
use crate::unity_engine::timeline::marker::Marker;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsemarker/MyRoomSEMarker.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSEMarker")]
#[parent(crate::unity_engine::timeline::marker::Marker)]
pub struct MyRoomSEMarker {
    #[rename(name = "EventName")]
    pub event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-myroomsemarker")]
#[::unity2::methods]
impl MyRoomSEMarker {
    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> crate::unity_engine::propertyname::PropertyName;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomsemarker")]
impl MyRoomSEMarker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSEMarker),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSEMarkerMethods>::ctor(this);
        this
    }
}
