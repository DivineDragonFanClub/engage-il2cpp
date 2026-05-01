
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::marker::IMarker;
use crate::unity_engine::timeline::marker::Marker;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomeffectmarker/MyRoomEffectMarker.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomEffectMarker")]
#[parent(crate::unity_engine::timeline::marker::Marker)]
pub struct MyRoomEffectMarker {
    #[rename(name = "EffectName")]
    pub effect_name: ::unity2::Il2CppString,
    #[rename(name = "duration")]
    pub duration: f32,
}

#[cfg(feature = "app-myroomeffectmarker")]
#[::unity2::methods]
impl MyRoomEffectMarker {
    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> crate::unity_engine::propertyname::PropertyName;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomeffectmarker")]
impl MyRoomEffectMarker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomEffectMarker),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomEffectMarkerMethods>::ctor(this);
        this
    }
}
