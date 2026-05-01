
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/u2d/spritedataaccessextensions/SpriteDataAccessExtensions.md")))]
#[::unity2::class(namespace = "UnityEngine.U2D", name = "SpriteDataAccessExtensions")]
#[parent(crate::system::object::Object)]
pub struct SpriteDataAccessExtensions {}

#[cfg(feature = "unity_engine-u2d-spritedataaccessextensions")]
#[::unity2::methods]
impl SpriteDataAccessExtensions {
    #[method(name = "GetIndicesInfo", args = 1)]
    pub fn get_indices_info(
        sprite: crate::unity_engine::sprite::Sprite,
    ) -> crate::unity_engine::u2d::spritechannelinfo::SpriteChannelInfo;

    #[method(name = "GetChannelInfo", args = 2)]
    pub fn get_channel_info(
        sprite: crate::unity_engine::sprite::Sprite,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> crate::unity_engine::u2d::spritechannelinfo::SpriteChannelInfo;

    #[method(name = "GetIndicesInfo_Injected", args = 2)]
    pub fn get_indices_info_injected(
        sprite: crate::unity_engine::sprite::Sprite,
        ret: crate::unity_engine::u2d::spritechannelinfo::SpriteChannelInfo,
    ) -> ();

    #[method(name = "GetChannelInfo_Injected", args = 3)]
    pub fn get_channel_info_injected(
        sprite: crate::unity_engine::sprite::Sprite,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        ret: crate::unity_engine::u2d::spritechannelinfo::SpriteChannelInfo,
    ) -> ();
}
