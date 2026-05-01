
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/collectionextensions/CollectionExtensions.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "CollectionExtensions"
)]
#[parent(crate::system::object::Object)]
pub struct CollectionExtensions {}
