
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamecurve/GameCurve.md")))]
#[::unity2::class(namespace = "App", name = "GameCurve")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct GameCurve {
    #[rename(name = "m_Curve")]
    pub m_curve: crate::unity_engine::animationcurve::AnimationCurve,
}

#[cfg(feature = "app-gamecurve")]
#[::unity2::methods]
impl GameCurve {
    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, time: f32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gamecurve")]
impl GameCurve {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameCurve),
                ::core::stringify!(new),
            )
        });
        <Self as IGameCurveMethods>::ctor(this);
        this
    }
}
