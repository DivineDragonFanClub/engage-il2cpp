
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/characterfactory/CharacterFactory.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "CharacterFactory")]
#[parent(crate::system::object::Object)]
pub struct CharacterFactory {}
