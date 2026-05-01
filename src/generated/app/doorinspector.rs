
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::pokeinspector::IPokeInspector;
use crate::app::pokeinspector::PokeInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/doorinspector/DoorInspector.md")))]
#[::unity2::class(namespace = "App", name = "DoorInspector")]
#[parent(crate::app::pokeinspector::PokeInspector)]
pub struct DoorInspector {}

#[cfg(feature = "app-doorinspector")]
#[::unity2::methods]
impl DoorInspector {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRange", args = 0)]
    pub fn get_range(self) -> i32;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "CanBreakable", args = 1)]
    pub fn can_breakable(self, force: crate::app::force::Force_Type) -> bool;

    #[method(name = "PreCall", args = 1)]
    pub fn pre_call(self, super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-doorinspector")]
impl DoorInspector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DoorInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IDoorInspectorMethods>::ctor(this);
        this
    }
}
