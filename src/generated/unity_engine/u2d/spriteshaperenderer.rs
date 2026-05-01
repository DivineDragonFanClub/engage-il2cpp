
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::renderer::IRenderer;
use crate::unity_engine::renderer::Renderer;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/u2d/spriteshaperenderer/SpriteShapeRenderer.md")))]
#[::unity2::class(namespace = "UnityEngine.U2D", name = "SpriteShapeRenderer")]
#[parent(crate::unity_engine::renderer::Renderer)]
pub struct SpriteShapeRenderer {}
