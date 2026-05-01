
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::pokeinspector::IPokeInspector;
use crate::app::pokeinspector::PokeInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tboxinspector/TboxInspector.md")))]
#[::unity2::class(namespace = "App", name = "TboxInspector")]
#[parent(crate::app::pokeinspector::PokeInspector)]
pub struct TboxInspector {}

#[cfg(feature = "app-tboxinspector")]
#[::unity2::methods]
impl TboxInspector {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRange", args = 0)]
    pub fn get_range(self) -> i32;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;
}

#[cfg(feature = "app-tboxinspector")]
impl TboxInspector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TboxInspector),
                ::core::stringify!(new),
            )
        });
        <Self as ITboxInspectorMethods>::ctor(this);
        this
    }
}
