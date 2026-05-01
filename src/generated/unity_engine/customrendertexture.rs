
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendertexture::IRenderTexture;
use crate::unity_engine::rendertexture::RenderTexture;
use crate::unity_engine::texture::ITexture;
use crate::unity_engine::texture::Texture;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/customrendertexture/CustomRenderTexture.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CustomRenderTexture")]
#[parent(crate::unity_engine::rendertexture::RenderTexture)]
pub struct CustomRenderTexture {}
