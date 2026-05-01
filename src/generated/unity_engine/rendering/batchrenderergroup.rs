
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/batchrenderergroup/BatchRendererGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "BatchRendererGroup")]
#[parent(crate::system::object::Object)]
pub struct BatchRendererGroup {
    #[rename(name = "m_GroupHandle")]
    pub m_group_handle: ::unity2::IntPtr,
    #[rename(name = "m_PerformCulling")]
    pub m_perform_culling:
        crate::unity_engine::rendering::batchrenderergroup::BatchRendererGroup_OnPerformCulling,
}

#[cfg(feature = "unity_engine-rendering-batchrenderergroup")]
#[::unity2::methods]
impl BatchRendererGroup {
    #[method(name = "InvokeOnPerformCulling", args = 3)]
    pub fn invoke_on_perform_culling(
        group: crate::unity_engine::rendering::batchrenderergroup::BatchRendererGroup,
        context : crate :: unity_engine :: rendering :: batchrenderercullingoutput :: BatchRendererCullingOutput,
        lod_parameters: crate::unity_engine::rendering::lodparameters::LODParameters,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/batchrenderergroup/BatchRendererGroup_OnPerformCulling.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering",
    name = "BatchRendererGroup.OnPerformCulling"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct BatchRendererGroup_OnPerformCulling {}

#[cfg(feature = "unity_engine-rendering-batchrenderergroup")]
#[::unity2::methods]
impl BatchRendererGroup_OnPerformCulling {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();
}

#[cfg(feature = "unity_engine-rendering-batchrenderergroup")]
impl BatchRendererGroup_OnPerformCulling {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BatchRendererGroup_OnPerformCulling),
                ::core::stringify!(new),
            )
        });
        <Self as IBatchRendererGroup_OnPerformCullingMethods>::ctor(this, object, method);
        this
    }
}
