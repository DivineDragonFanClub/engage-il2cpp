
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/personinspector/PersonInspector.md")))]
#[::unity2::class(namespace = "App", name = "PersonInspector")]
#[parent(crate::app::mapinspector::MapInspector)]
pub struct PersonInspector {
    #[rename(name = "m_Person")]
    pub m_person: i32,
}

#[cfg(feature = "app-personinspector")]
#[::unity2::methods]
impl PersonInspector {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "IsEanble", args = 1)]
    pub fn is_eanble(self, person: i32) -> bool;
}

#[cfg(feature = "app-personinspector")]
impl PersonInspector {
    pub fn new(
        kind: crate::app::mapinspector::MapInspector_Kind,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersonInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IPersonInspectorMethods>::ctor(this, kind, args);
        this
    }
}
