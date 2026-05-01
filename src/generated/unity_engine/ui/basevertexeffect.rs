
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/basevertexeffect/BaseVertexEffect.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "BaseVertexEffect")]
#[parent(crate::system::object::Object)]
pub struct BaseVertexEffect {}

#[cfg(feature = "unity_engine-ui-basevertexeffect")]
#[::unity2::methods]
impl BaseVertexEffect {
    #[method(name = "ModifyVertices", args = 1)]
    pub fn modify_vertices(
        self,
        vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-basevertexeffect")]
impl BaseVertexEffect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseVertexEffect),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseVertexEffectMethods>::ctor(this);
        this
    }
}
