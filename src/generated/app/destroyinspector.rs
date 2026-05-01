
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::pokeinspector::IPokeInspector;
use crate::app::pokeinspector::PokeInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/destroyinspector/DestroyInspector.md")))]
#[::unity2::class(namespace = "App", name = "DestroyInspector")]
#[parent(crate::app::pokeinspector::PokeInspector)]
pub struct DestroyInspector {}

#[cfg(feature = "app-destroyinspector")]
#[::unity2::methods]
impl DestroyInspector {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetLabel", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRange", args = 0)]
    pub fn get_range(self) -> i32;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "PreCall", args = 1)]
    pub fn pre_call(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetGlobalKey", args = 0)]
    pub fn get_global_key(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-destroyinspector")]
impl DestroyInspector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DestroyInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IDestroyInspectorMethods>::ctor(this);
        this
    }
}
