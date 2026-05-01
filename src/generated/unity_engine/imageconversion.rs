
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/imageconversion/ImageConversion.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ImageConversion")]
#[parent(crate::system::object::Object)]
pub struct ImageConversion {}

#[cfg(feature = "unity_engine-imageconversion")]
#[::unity2::methods]
impl ImageConversion {
    #[method(name = "EncodeToPNG", args = 1)]
    pub fn encode_to_png(tex: crate::unity_engine::texture2d::Texture2D) -> ::unity2::Array<u8>;

    #[method(name = "EncodeToJPG", args = 2)]
    pub fn encode_to_jpg(
        tex: crate::unity_engine::texture2d::Texture2D,
        quality: i32,
    ) -> ::unity2::Array<u8>;

    #[method(name = "EncodeToJPG", args = 1)]
    pub fn encode_to_jpg_2(tex: crate::unity_engine::texture2d::Texture2D) -> ::unity2::Array<u8>;

    #[method(name = "LoadImage", args = 3)]
    pub fn load_image(
        tex: crate::unity_engine::texture2d::Texture2D,
        data: ::unity2::Array<u8>,
        mark_non_readable: bool,
    ) -> bool;

    #[method(name = "LoadImage", args = 2)]
    pub fn load_image_2(
        tex: crate::unity_engine::texture2d::Texture2D,
        data: ::unity2::Array<u8>,
    ) -> bool;
}
