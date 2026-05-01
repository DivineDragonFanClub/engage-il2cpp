
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::personinspector::IPersonInspector;
use crate::app::personinspector::PersonInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcommandprepareinspector/UnitCommandPrepareInspector.md")))]
#[::unity2::class(namespace = "App", name = "UnitCommandPrepareInspector")]
#[parent(crate::app::personinspector::PersonInspector)]
pub struct UnitCommandPrepareInspector {}

#[cfg(feature = "app-unitcommandprepareinspector")]
#[::unity2::methods]
impl UnitCommandPrepareInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-unitcommandprepareinspector")]
impl UnitCommandPrepareInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCommandPrepareInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCommandPrepareInspectorMethods>::ctor(this, args);
        this
    }
}
