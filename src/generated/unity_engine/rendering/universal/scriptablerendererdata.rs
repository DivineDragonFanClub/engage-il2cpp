
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/scriptablerendererdata/ScriptableRendererData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "ScriptableRendererData"
)]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct ScriptableRendererData {
# [rename (name = "m_RendererFeatures")] pub m_renderer_features : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: rendering :: universal :: scriptablerendererfeature :: ScriptableRendererFeature > ,
# [rename (name = "m_RendererFeatureMap")] pub m_renderer_feature_map : crate :: system :: collections :: generic :: list_1 :: List_1 < i64 > ,
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerendererdata")]
#[::unity2::methods]
impl ScriptableRendererData {
    #[method(name = "get_isInvalidated", args = 0)]
    pub fn get_is_invalidated(self) -> bool;

    #[method(name = "set_isInvalidated", args = 1)]
    pub fn set_is_invalidated(self, value: bool) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(
        self,
    ) -> crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;

    #[method(name = "get_rendererFeatures", args = 0)]
    pub fn get_renderer_features (self ,) -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: rendering :: universal :: scriptablerendererfeature :: ScriptableRendererFeature > ;

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty(self) -> ();

    #[method(name = "InternalCreateRenderer", args = 0)]
    pub fn internal_create_renderer(
        self,
    ) -> crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerendererdata")]
impl ScriptableRendererData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptableRendererData),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptableRendererDataMethods>::ctor(this);
        this
    }
}
