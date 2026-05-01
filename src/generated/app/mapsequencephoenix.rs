
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencephoenix/MapSequencePhoenix_ProcAppear.md")))]
#[::unity2::class(namespace = "App", name = "MapSequencePhoenix.ProcAppear")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequencePhoenix_ProcAppear {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-mapsequencephoenix")]
#[::unity2::methods]
impl MapSequencePhoenix_ProcAppear {
    #[method(name = "Create", args = 2)]
    pub fn create(super_: crate::app::procinst::ProcInst, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "LoadDone", args = 0)]
    pub fn load_done(self) -> ();

    #[method(name = "WarpIn", args = 0)]
    pub fn warp_in(self) -> ();
}

#[cfg(feature = "app-mapsequencephoenix")]
impl MapSequencePhoenix_ProcAppear {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequencePhoenix_ProcAppear),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequencePhoenix_ProcAppearMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencephoenix/MapSequencePhoenix.md")))]
#[::unity2::class(namespace = "App", name = "MapSequencePhoenix")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequencePhoenix {}

#[cfg(feature = "app-mapsequencephoenix")]
#[::unity2::methods]
impl MapSequencePhoenix {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MoveCamera", args = 0)]
    pub fn move_camera(self) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "IsExecuting", args = 0)]
    pub fn is_executing(self) -> bool;

    #[method(name = "Postexecute", args = 0)]
    pub fn postexecute(self) -> ();

    #[method(name = "IsPhoenixUnit", args = 1)]
    pub fn is_phoenix_unit(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetPhoenixFirst", args = 0)]
    pub fn get_phoenix_first() -> crate::app::unit::Unit;

    #[method(name = "GetPhoenixPosition", args = 3)]
    pub fn get_phoenix_position(dst_x: i32, dst_z: i32, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencephoenix")]
impl MapSequencePhoenix {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequencePhoenix),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequencePhoenixMethods>::ctor(this);
        this
    }
}
