
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::app::unitinspector::IUnitInspector;
use crate::app::unitinspector::UnitInspector;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dieinspector/DieInspector.md")))]
#[::unity2::class(namespace = "App", name = "DieInspector")]
#[parent(crate::app::unitinspector::UnitInspector)]
pub struct DieInspector {}

#[cfg(feature = "app-dieinspector")]
#[::unity2::methods]
impl DieInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-dieinspector")]
impl DieInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DieInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IDieInspectorMethods>::ctor(this, args);
        this
    }
}
