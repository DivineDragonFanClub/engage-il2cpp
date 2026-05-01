
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/water/reflectionmanager/ReflectionManager.md")))]
#[::unity2::class(namespace = "App.Water", name = "ReflectionManager")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ReflectionManager {
    #[rename(name = "WaterMeshRoot")]
    pub water_mesh_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_renderedList")]
    pub m_rendered_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-water-reflectionmanager")]
#[::unity2::methods]
impl ReflectionManager {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "RemoveNonReferenceCamera", args = 0)]
    pub fn remove_non_reference_camera(self) -> ();

    #[method(name = "ClearRenderedList", args = 0)]
    pub fn clear_rendered_list(self) -> ();

    #[method(name = "RenderReflection", args = 3)]
    pub fn render_reflection(
        self,
        tr: crate::unity_engine::transform::Transform,
        cam: crate::unity_engine::camera::Camera,
        number: i16,
    ) -> bool;

    #[method(name = "AddReflectionCamera", args = 1)]
    pub fn add_reflection_camera(
        self,
        obj_camera: crate::unity_engine::gameobject::GameObject,
    ) -> crate::app::water::reflectioncamera::ReflectionCamera;

    #[method(name = "GetReflectionTexture", args = 1)]
    pub fn get_reflection_texture(
        self,
        number: i16,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetCameraName", args = 1)]
    pub fn get_camera_name(self, number: i16) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-water-reflectionmanager")]
impl ReflectionManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionManager),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionManagerMethods>::ctor(this);
        this
    }
}
