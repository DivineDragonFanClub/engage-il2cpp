
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencemove/MapSequenceMove.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceMove")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapsequencemove :: MapSequenceMove >)]
pub struct MapSequenceMove {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-mapsequencemove")]
#[::unity2::methods]
impl MapSequenceMove {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-mapsequencemove")]
impl MapSequenceMove {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceMove),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceMoveMethods>::ctor(this, unit);
        this
    }
}
