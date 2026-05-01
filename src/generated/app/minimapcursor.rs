
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/minimapcursor/MiniMapCursor.md")))]
#[::unity2::class(namespace = "App", name = "MiniMapCursor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MiniMapCursor {
    #[rename(name = "m_CursorLT")]
    pub m_cursor_lt: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorRT")]
    pub m_cursor_rt: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorLB")]
    pub m_cursor_lb: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorRB")]
    pub m_cursor_rb: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_OrgCursorLT")]
    pub m_org_cursor_lt: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_OrgCursorRT")]
    pub m_org_cursor_rt: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_OrgCursorLB")]
    pub m_org_cursor_lb: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_OrgCursorRB")]
    pub m_org_cursor_rb: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-minimapcursor")]
#[::unity2::methods]
impl MiniMapCursor {
    #[method(name = "get_Scale", args = 0)]
    pub fn get_scale(self) -> f32;

    #[method(name = "set_Scale", args = 1)]
    pub fn set_scale(self, value: f32) -> ();

    #[method(name = "get_CalculatedGridSize", args = 0)]
    pub fn get_calculated_grid_size(self) -> f32;

    #[method(name = "set_CalculatedGridSize", args = 1)]
    pub fn set_calculated_grid_size(self, value: f32) -> ();

    #[method(name = "get_IsCursorVisible", args = 0)]
    pub fn get_is_cursor_visible(self) -> bool;

    #[method(name = "set_IsCursorVisible", args = 1)]
    pub fn set_is_cursor_visible(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "TryInitializeParts", args = 0)]
    pub fn try_initialize_parts(self) -> ();

    #[method(name = "CalculateCursorPosition", args = 1)]
    pub fn calculate_cursor_position(self, grid_size: f32)
        -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "GetCenter", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetPosition", args = 1)]
    pub fn get_position(
        obj: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-minimapcursor")]
impl MiniMapCursor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MiniMapCursor),
                ::core::stringify!(new),
            )
        });
        <Self as IMiniMapCursorMethods>::ctor(this);
        this
    }
}
