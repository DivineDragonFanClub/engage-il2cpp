
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eachinspector/EachInspector.md")))]
#[::unity2::class(namespace = "App", name = "EachInspector")]
#[parent(crate::app::mapinspector::MapInspector)]
pub struct EachInspector {
    #[rename(name = "m_FromPerson")]
    pub m_from_person: i32,
    #[rename(name = "m_FromForce")]
    pub m_from_force: i32,
    #[rename(name = "m_ToPerson")]
    pub m_to_person: i32,
    #[rename(name = "m_ToForce")]
    pub m_to_force: i32,
    #[rename(name = "m_Both")]
    pub m_both: bool,
}

#[cfg(feature = "app-eachinspector")]
#[::unity2::methods]
impl EachInspector {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "IsEach", args = 4)]
    pub fn is_each(self, from_person: i32, from_force: i32, to_person: i32, to_force: i32) -> bool;

    #[method(name = "IsEanble", args = 4)]
    pub fn is_eanble(
        self,
        from_person: i32,
        from_force: i32,
        to_person: i32,
        to_force: i32,
    ) -> bool;
}

#[cfg(feature = "app-eachinspector")]
impl EachInspector {
    pub fn new(
        kind: crate::app::mapinspector::MapInspector_Kind,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EachInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IEachInspectorMethods>::ctor(this, kind, args);
        this
    }
}
