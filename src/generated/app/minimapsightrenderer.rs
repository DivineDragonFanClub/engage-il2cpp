
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::graphic::Graphic;
use crate::unity_engine::ui::graphic::IGraphic;
use crate::unity_engine::ui::maskablegraphic::IMaskableGraphic;
use crate::unity_engine::ui::maskablegraphic::MaskableGraphic;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minimapsightrenderer/MiniMapSightRenderer.md")))]
#[::unity2::class(namespace = "App", name = "MiniMapSightRenderer")]
#[parent(crate::unity_engine::ui::maskablegraphic::MaskableGraphic)]
pub struct MiniMapSightRenderer {
    #[rename(name = "m_Texture")]
    pub m_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_CanvasRenderer")]
    pub m_canvas_renderer: crate::unity_engine::canvasrenderer::CanvasRenderer,
    #[rename(name = "m_SightFillColor")]
    pub m_sight_fill_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Mesh")]
    pub m_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_Vertices")]
    pub m_vertices:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_Colors")]
    pub m_colors:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::color::Color>,
    #[rename(name = "m_UVs")]
    pub m_u_vs:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector2::Vector2>,
    #[rename(name = "m_Indices")]
    pub m_indices: crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "VERTEX_CAPACITY")]
    pub vertex_capacity: i32,
    #[rename(name = "vtxOffsets")]
    pub vtx_offsets: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "app-minimapsightrenderer")]
#[::unity2::methods]
impl MiniMapSightRenderer {
    #[method(name = "get_Texture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "set_Texture", args = 1)]
    pub fn set_texture(self, value: crate::unity_engine::texture2d::Texture2D) -> ();

    #[method(name = "get_CanvasRenderer", args = 0)]
    pub fn get_canvas_renderer(self) -> crate::unity_engine::canvasrenderer::CanvasRenderer;

    #[method(name = "set_GridSize", args = 1)]
    pub fn set_grid_size(self, value: f32) -> ();

    #[method(name = "get_GridSize", args = 0)]
    pub fn get_grid_size(self) -> f32;

    #[method(name = "TryInitializeData", args = 0)]
    pub fn try_initialize_data(self) -> ();

    #[method(name = "GetMapImage", args = 0)]
    pub fn get_map_image(self) -> crate::app::mapimage::MapImage;

    #[method(name = "UpdateGeometry", args = 0)]
    pub fn update_geometry(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "CreateTerrainMesh", args = 0)]
    pub fn create_terrain_mesh(self) -> ();

    #[method(name = "CreateTerrainMesh", args = 1)]
    pub fn create_terrain_mesh_2(self, map_image: crate::app::mapimage::MapImage) -> ();

    #[method(name = "SetMeshToRenderer", args = 0)]
    pub fn set_mesh_to_renderer(self) -> ();

    #[method(name = "UpdateMapSize", args = 0)]
    pub fn update_map_size(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-minimapsightrenderer")]
impl MiniMapSightRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MiniMapSightRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as IMiniMapSightRendererMethods>::ctor(this);
        this
    }
}
