
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playableextensions/PlayableExtensions.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "PlayableExtensions")]
#[parent(crate::system::object::Object)]
pub struct PlayableExtensions {}
