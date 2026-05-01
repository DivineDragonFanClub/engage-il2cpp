
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chapterrecord/ChapterRecord.md")))]
#[::unity2::class(namespace = "App", name = "ChapterRecord")]
#[parent(crate::system::object::Object)]
pub struct ChapterRecord {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::app::chapterrecord::ChapterRecord_Record,
    >,
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::chapterrecord::ChapterRecord_Record,
    >,
}

#[cfg(feature = "app-chapterrecord")]
#[::unity2::methods]
impl ChapterRecord {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "Find", args = 1)]
    pub fn find(
        self,
        cid: ::unity2::Il2CppString,
    ) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "Find", args = 1)]
    pub fn find_2(
        self,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(
        self,
        record: crate::app::chapterrecord::ChapterRecord_Record,
    ) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add_2(
        self,
        cid: ::unity2::Il2CppString,
    ) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add_3(
        self,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> crate::app::chapterrecord::ChapterRecord_Record;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-chapterrecord")]
impl ChapterRecord {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChapterRecord),
                ::core::stringify!(new),
            )
        });
        <Self as IChapterRecordMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chapterrecord/ChapterRecord_Record.md")))]
#[::unity2::class(namespace = "App", name = "ChapterRecord.Record")]
#[parent(crate::system::object::Object)]
pub struct ChapterRecord_Record {
    #[rename(name = "Cid")]
    pub cid: ::unity2::Il2CppString,
    #[rename(name = "MvpPid")]
    pub mvp_pid: ::unity2::Il2CppString,
    #[rename(name = "MvpJid")]
    pub mvp_jid: ::unity2::Il2CppString,
    #[rename(name = "MvpGid")]
    pub mvp_gid: ::unity2::Il2CppString,
    #[rename(name = "ClearTime")]
    pub clear_time: f32,
    #[rename(name = "ClearTurn")]
    pub clear_turn: i32,
}

#[cfg(feature = "app-chapterrecord")]
#[::unity2::methods]
impl ChapterRecord_Record {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, cid: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-chapterrecord")]
impl ChapterRecord_Record {
    pub fn new(cid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChapterRecord_Record),
                ::core::stringify!(new),
            )
        });
        <Self as IChapterRecord_RecordMethods>::ctor(this, cid);
        this
    }
}
