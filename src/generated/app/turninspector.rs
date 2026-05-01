
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::app::turncommoninspector::ITurnCommonInspector;
use crate::app::turncommoninspector::TurnCommonInspector;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/turninspector/TurnInspector.md")))]
#[::unity2::class(namespace = "App", name = "TurnInspector")]
#[parent(crate::app::turncommoninspector::TurnCommonInspector)]
pub struct TurnInspector {}

#[cfg(feature = "app-turninspector")]
#[::unity2::methods]
impl TurnInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-turninspector")]
impl TurnInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TurnInspector),
                ::core::stringify!(new),
            )
        });
        <Self as ITurnInspectorMethods>::ctor(this, args);
        this
    }
}
