
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::app::turncommoninspector::ITurnCommonInspector;
use crate::app::turncommoninspector::TurnCommonInspector;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/turnendinspector/TurnEndInspector.md")))]
#[::unity2::class(namespace = "App", name = "TurnEndInspector")]
#[parent(crate::app::turncommoninspector::TurnCommonInspector)]
pub struct TurnEndInspector {}

#[cfg(feature = "app-turnendinspector")]
#[::unity2::methods]
impl TurnEndInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-turnendinspector")]
impl TurnEndInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TurnEndInspector),
                ::core::stringify!(new),
            )
        });
        <Self as ITurnEndInspectorMethods>::ctor(this, args);
        this
    }
}
