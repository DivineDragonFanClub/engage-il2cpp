
use crate::app::dynamicmesh::DynamicMesh;
use crate::app::dynamicmesh::IDynamicMesh;
use crate::app::map::IMap_CellMesh;
use crate::app::map::Map_CellMesh;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gridmesh/GridMesh.md")))]
#[::unity2::class(namespace = "App", name = "GridMesh")]
#[parent(crate::app::map::Map_CellMesh)]
pub struct GridMesh {}

#[cfg(feature = "app-gridmesh")]
#[::unity2::methods]
impl GridMesh {
    #[method(name = "DrawGrid", args = 4)]
    pub fn draw_grid(
        self,
        cell_map: crate::app::mapheight::MapHeight_CellMap,
        x: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gridmesh")]
impl GridMesh {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GridMesh),
                ::core::stringify!(new),
            )
        });
        <Self as IGridMeshMethods>::ctor(this);
        this
    }
}
