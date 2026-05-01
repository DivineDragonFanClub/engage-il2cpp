
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapranking/MapRanking.md")))]
#[::unity2::class(namespace = "App", name = "MapRanking")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapranking :: MapRanking >)]
pub struct MapRanking {
    #[rename(name = "pairs")]
    pub pairs: crate::system::collections::generic::list_1::List_1<
        crate::app::nexranking::NexRanking_Data,
    >,
}

#[cfg(feature = "app-mapranking")]
#[::unity2::methods]
impl MapRanking {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "RegisterUnit", args = 0)]
    pub fn register_unit(self) -> ();

    #[method(name = "Upload", args = 1)]
    pub fn upload(self, super_: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "GetChapter", args = 0)]
    pub fn get_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "DbgDump", args = 0)]
    pub fn dbg_dump(self) -> ();

    #[method(name = "ClearNetSucceeded", args = 0)]
    pub fn clear_net_succeeded(self) -> ();

    #[method(name = "get_IsNetSucceeded", args = 0)]
    pub fn get_is_net_succeeded(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapranking")]
impl MapRanking {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapRanking),
                ::core::stringify!(new),
            )
        });
        <Self as IMapRankingMethods>::ctor(this);
        this
    }
}
