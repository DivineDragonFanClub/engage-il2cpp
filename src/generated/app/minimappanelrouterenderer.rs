
use crate::app::minimappanelbase::IMiniMapPanelBase;
use crate::app::minimappanelbase::MiniMapPanelBase;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minimappanelrouterenderer/MiniMapPanelRouteRenderer.md")))]
#[::unity2::class(namespace = "App", name = "MiniMapPanelRouteRenderer")]
#[parent(crate::app::minimappanelbase::MiniMapPanelBase)]
pub struct MiniMapPanelRouteRenderer {}

#[cfg(feature = "app-minimappanelrouterenderer")]
#[::unity2::methods]
impl MiniMapPanelRouteRenderer {
    #[method(name = "CreatePanelMesh", args = 0)]
    pub fn create_panel_mesh(self) -> ();

    #[method(name = "GetMapPanelMaterials", args = 0)]
    pub fn get_map_panel_materials(
        self,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "CreatePanelRouteMesh", args = 0)]
    pub fn create_panel_route_mesh(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-minimappanelrouterenderer")]
impl MiniMapPanelRouteRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MiniMapPanelRouteRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as IMiniMapPanelRouteRendererMethods>::ctor(this);
        this
    }
}
