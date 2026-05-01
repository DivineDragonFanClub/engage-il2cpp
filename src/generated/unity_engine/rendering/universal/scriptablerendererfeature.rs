
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/scriptablerendererfeature/ScriptableRendererFeature.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "ScriptableRendererFeature"
)]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct ScriptableRendererFeature {
    #[rename(name = "m_Active")]
    pub m_active: bool,
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerendererfeature")]
#[::unity2::methods]
impl ScriptableRendererFeature {
    #[method(name = "get_isActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "AddRenderPasses", args = 2)]
    pub fn add_render_passes(
        self,
        renderer: crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, active: bool) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerendererfeature")]
impl ScriptableRendererFeature {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptableRendererFeature),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptableRendererFeatureMethods>::ctor(this);
        this
    }
}
