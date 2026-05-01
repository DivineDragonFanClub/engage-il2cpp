
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::pokeinspector::IPokeInspector;
use crate::app::pokeinspector::PokeInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/waypointinspector/WaypointInspector.md")))]
#[::unity2::class(namespace = "App", name = "WaypointInspector")]
#[parent(crate::app::pokeinspector::PokeInspector)]
pub struct WaypointInspector {}

#[cfg(feature = "app-waypointinspector")]
#[::unity2::methods]
impl WaypointInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRange", args = 0)]
    pub fn get_range(self) -> i32;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;
}

#[cfg(feature = "app-waypointinspector")]
impl WaypointInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaypointInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IWaypointInspectorMethods>::ctor(this, args);
        this
    }
}
