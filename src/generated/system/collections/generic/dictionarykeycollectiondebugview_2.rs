
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionarykeycollectiondebugview_2/DictionaryKeyCollectionDebugView_2.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "DictionaryKeyCollectionDebugView`2"
)]
pub struct DictionaryKeyCollectionDebugView_2<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
> {}
