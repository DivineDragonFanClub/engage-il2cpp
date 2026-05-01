
use crate::app::singletonscriptableobject_1::ISingletonScriptableObject_1;
use crate::app::singletonscriptableobject_1::SingletonScriptableObject_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godcolor/GodColor.md")))]
#[::unity2::class(namespace = "App", name = "GodColor")]
# [parent (crate :: app :: singletonscriptableobject_1 :: SingletonScriptableObject_1 < crate :: app :: godcolor :: GodColor >)]
pub struct GodColor {
    #[rename(name = "マルス")]
    pub _unnamed: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-godcolor")]
#[::unity2::methods]
impl GodColor {
    #[method(name = "GetGodColor", args = 1)]
    pub fn get_god_color(
        self,
        god: crate::app::goddata::GodData,
    ) -> crate::unity_engine::color::Color;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godcolor")]
impl GodColor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodColor),
                ::core::stringify!(new),
            )
        });
        <Self as IGodColorMethods>::ctor(this);
        this
    }
}
