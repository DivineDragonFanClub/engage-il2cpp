
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappanelroute/MapPanelRoute.md")))]
#[::unity2::class(namespace = "App", name = "MapPanelRoute")]
pub struct MapPanelRoute {
    #[rename(name = "m_Routes")]
    pub m_routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
}

#[cfg(feature = "app-mappanelroute")]
#[::unity2::methods]
impl MapPanelRoute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetupRoute", args = 0)]
    pub fn setup_route(self) -> ();

    #[method(name = "SetupResumeRoute", args = 1)]
    pub fn setup_resume_route(self, routes: ::unity2::Array<crate::app::dir_2::Dir_Type>) -> ();

    #[method(name = "SetTransparent", args = 1)]
    pub fn set_transparent(self, enable: bool) -> ();

    #[method(name = "Save", args = 1)]
    pub fn save(self, routes: ::unity2::Array<crate::app::dir_2::Dir_Type>) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "IsRoute", args = 0)]
    pub fn is_route(self) -> bool;

    #[method(name = "GetUV", args = 2)]
    pub fn get_uv(
        self,
        old_dir: crate::app::dir_2::Dir_Type,
        cur_dir: crate::app::dir_2::Dir_Type,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "AddVerticalVertex", args = 5)]
    pub fn add_vertical_vertex(
        self,
        old_dir: crate::app::dir_2::Dir_Type,
        old_x: i32,
        old_z: i32,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "AddHeadVertex", args = 3)]
    pub fn add_head_vertex(self, old_dir: crate::app::dir_2::Dir_Type, x: i32, z: i32) -> ();

    #[method(name = "AddCell", args = 3)]
    pub fn add_cell(self, x: i32, z: i32, uv: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "AddCell", args = 5)]
    pub fn add_cell_2(
        self,
        x: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
        uv0: crate::unity_engine::vector2::Vector2,
        uv2: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_Routes", args = 0)]
    pub fn get_routes(self) -> ::unity2::Array<crate::app::dir_2::Dir_Type>;
}

#[cfg(feature = "app-mappanelroute")]
impl MapPanelRoute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPanelRoute),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPanelRouteMethods>::ctor(this);
        this
    }
}
