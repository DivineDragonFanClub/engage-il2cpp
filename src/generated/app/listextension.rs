
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/listextension/ListExtension.md")))]
#[::unity2::class(namespace = "App", name = "ListExtension")]
#[parent(crate::system::object::Object)]
pub struct ListExtension {}
