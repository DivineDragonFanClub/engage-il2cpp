
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::app::unitinspector::IUnitInspector;
use crate::app::unitinspector::UnitInspector;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fixedinspector/FixedInspector.md")))]
#[::unity2::class(namespace = "App", name = "FixedInspector")]
#[parent(crate::app::unitinspector::UnitInspector)]
pub struct FixedInspector {}

#[cfg(feature = "app-fixedinspector")]
#[::unity2::methods]
impl FixedInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-fixedinspector")]
impl FixedInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FixedInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IFixedInspectorMethods>::ctor(this, args);
        this
    }
}
