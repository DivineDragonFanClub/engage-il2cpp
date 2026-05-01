
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionaryvaluecollectiondebugview_2/DictionaryValueCollectionDebugView_2.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "DictionaryValueCollectionDebugView`2"
)]
pub struct DictionaryValueCollectionDebugView_2<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
> {}
