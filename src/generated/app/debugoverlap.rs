
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugoverlap/DebugOverlap.md")))]
#[::unity2::class(namespace = "App", name = "DebugOverlap")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: debugoverlap :: DebugOverlap >)]
pub struct DebugOverlap {
    #[rename(name = "m_Shader")]
    pub m_shader: crate::unity_engine::shader::Shader,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_ClearFlags")]
    pub m_clear_flags: crate::unity_engine::cameraclearflags::CameraClearFlags,
    #[rename(name = "m_BackgroundColor")]
    pub m_background_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Renders")]
    pub m_renders: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::renderer::Renderer,
        ::unity2::Array<crate::unity_engine::material::Material>,
    >,
}

#[cfg(feature = "app-debugoverlap")]
#[::unity2::methods]
impl DebugOverlap {
    #[method(name = "GetMaterials", args = 1)]
    pub fn get_materials(
        self,
        render: crate::unity_engine::renderer::Renderer,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "SetMaterials", args = 2)]
    pub fn set_materials(
        self,
        render: crate::unity_engine::renderer::Renderer,
        materials: ::unity2::Array<crate::unity_engine::material::Material>,
    ) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugoverlap")]
impl DebugOverlap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugOverlap),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugOverlapMethods>::ctor(this);
        this
    }
}
