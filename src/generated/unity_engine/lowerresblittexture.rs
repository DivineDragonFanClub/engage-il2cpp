
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/lowerresblittexture/LowerResBlitTexture.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LowerResBlitTexture")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct LowerResBlitTexture {}

#[cfg(feature = "unity_engine-lowerresblittexture")]
#[::unity2::methods]
impl LowerResBlitTexture {
    #[method(name = "LowerResBlitTextureDontStripMe", args = 0)]
    pub fn lower_res_blit_texture_dont_strip_me(self) -> ();
}
