
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::universal::scriptablerendererfeature::IScriptableRendererFeature;
use crate::unity_engine::rendering::universal::scriptablerendererfeature::ScriptableRendererFeature;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/renderobjects/RenderObjects_CustomCameraSettings.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "RenderObjects.CustomCameraSettings"
)]
#[parent(crate::system::object::Object)]
pub struct RenderObjects_CustomCameraSettings {
    #[rename(name = "overrideCamera")]
    pub override_camera: bool,
    #[rename(name = "restoreCamera")]
    pub restore_camera: bool,
    #[rename(name = "offset")]
    pub offset: crate::unity_engine::vector4::Vector4,
    #[rename(name = "cameraFieldOfView")]
    pub camera_field_of_view: f32,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
#[::unity2::methods]
impl RenderObjects_CustomCameraSettings {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
impl RenderObjects_CustomCameraSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderObjects_CustomCameraSettings),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderObjects_CustomCameraSettingsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/renderobjects/RenderObjects_RenderObjectsSettings.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "RenderObjects.RenderObjectsSettings"
)]
#[parent(crate::system::object::Object)]
pub struct RenderObjects_RenderObjectsSettings {
# [rename (name = "passTag")] pub pass_tag : :: unity2 :: Il2CppString ,
# [rename (name = "Event")] pub event : crate :: unity_engine :: rendering :: universal :: renderpassevent :: RenderPassEvent ,
# [rename (name = "filterSettings")] pub filter_settings : crate :: unity_engine :: experimental :: rendering :: universal :: renderobjects :: RenderObjects_FilterSettings ,
# [rename (name = "overrideMaterial")] pub override_material : crate :: unity_engine :: material :: Material ,
# [rename (name = "overrideMaterialPassIndex")] pub override_material_pass_index : i32 ,
# [rename (name = "overrideDepthState")] pub override_depth_state : bool ,
# [rename (name = "depthCompareFunction")] pub depth_compare_function : crate :: unity_engine :: rendering :: comparefunction :: CompareFunction ,
# [rename (name = "enableWrite")] pub enable_write : bool ,
# [rename (name = "stencilSettings")] pub stencil_settings : crate :: unity_engine :: rendering :: universal :: stencilstatedata :: StencilStateData ,
# [rename (name = "cameraSettings")] pub camera_settings : crate :: unity_engine :: experimental :: rendering :: universal :: renderobjects :: RenderObjects_CustomCameraSettings ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
#[::unity2::methods]
impl RenderObjects_RenderObjectsSettings {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
impl RenderObjects_RenderObjectsSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderObjects_RenderObjectsSettings),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderObjects_RenderObjectsSettingsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/renderobjects/RenderObjects.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "RenderObjects"
)]
#[parent(
    crate::unity_engine::rendering::universal::scriptablerendererfeature::ScriptableRendererFeature
)]
pub struct RenderObjects {
# [rename (name = "settings")] pub settings : crate :: unity_engine :: experimental :: rendering :: universal :: renderobjects :: RenderObjects_RenderObjectsSettings ,
# [rename (name = "renderObjectsPass")] pub render_objects_pass : crate :: unity_engine :: experimental :: rendering :: universal :: renderobjectspass :: RenderObjectsPass ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
#[::unity2::methods]
impl RenderObjects {
    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "AddRenderPasses", args = 2)]
    pub fn add_render_passes(
        self,
        renderer: crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
impl RenderObjects {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderObjects),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderObjectsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/renderobjects/RenderObjects_FilterSettings.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "RenderObjects.FilterSettings"
)]
#[parent(crate::system::object::Object)]
pub struct RenderObjects_FilterSettings {
    #[rename(name = "RenderQueueType")]
    pub render_queue_type:
        crate::unity_engine::experimental::rendering::universal::renderqueuetype::RenderQueueType,
    #[rename(name = "LayerMask")]
    pub layer_mask: crate::unity_engine::layermask::LayerMask,
    #[rename(name = "PassNames")]
    pub pass_names: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
#[::unity2::methods]
impl RenderObjects_FilterSettings {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-renderobjects")]
impl RenderObjects_FilterSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderObjects_FilterSettings),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderObjects_FilterSettingsMethods>::ctor(this);
        this
    }
}
