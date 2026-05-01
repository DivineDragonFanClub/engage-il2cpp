
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/rectangularvertexclipper/RectangularVertexClipper.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "RectangularVertexClipper")]
#[parent(crate::system::object::Object)]
pub struct RectangularVertexClipper {
    #[rename(name = "m_WorldCorners")]
    pub m_world_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_CanvasCorners")]
    pub m_canvas_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "unity_engine-ui-rectangularvertexclipper")]
#[::unity2::methods]
impl RectangularVertexClipper {
    #[method(name = "GetCanvasRect", args = 2)]
    pub fn get_canvas_rect(
        self,
        t: crate::unity_engine::recttransform::RectTransform,
        c: crate::unity_engine::canvas::Canvas,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-rectangularvertexclipper")]
impl RectangularVertexClipper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RectangularVertexClipper),
                ::core::stringify!(new),
            )
        });
        <Self as IRectangularVertexClipperMethods>::ctor(this);
        this
    }
}
