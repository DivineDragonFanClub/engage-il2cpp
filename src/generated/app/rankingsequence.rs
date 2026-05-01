
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_UploadPersonMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.UploadPersonMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct RankingSequence_UploadPairsMenu_UploadPersonMenuItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_UploadPersonMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_UploadPersonMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_UploadPersonMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_UploadPersonMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_TopMenu_ChapterSelect.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.TopMenu.ChapterSelect")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct RankingSequence_TopMenu_ChapterSelect {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_TopMenu_ChapterSelect {
    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_TopMenu_ChapterSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_TopMenu_ChapterSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_TopMenu_ChapterSelectMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_UploadPairMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.UploadPairMenuItem"
)]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_UploadPairsMenu_UploadPairMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_UploadPairMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_UploadPairMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_UploadPairMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_UploadPairMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.RandomAddUploadPairsMenuItem"
)]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_RandomAddUploadPairsMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_AddUploadPairsMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.AddUploadPairsMenuItem"
)]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_UploadPairsMenu_AddUploadPairsMenuItem {
    #[rename(name = "m_UploadPersonMenuItem")]
    pub m_upload_person_menu_item:
        crate::app::rankingsequence::RankingSequence_UploadPairsMenu_UploadPersonMenuItem,
    #[rename(name = "m_UploadGodMenuItem")]
    pub m_upload_god_menu_item:
        crate::app::rankingsequence::RankingSequence_UploadPairsMenu_UploadGodMenuItem,
}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_AddUploadPairsMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        person_item : crate :: app :: rankingsequence :: RankingSequence_UploadPairsMenu_UploadPersonMenuItem,
        god_item: crate::app::rankingsequence::RankingSequence_UploadPairsMenu_UploadGodMenuItem,
    ) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_AddUploadPairsMenuItem {
    pub fn new(
        person_item : crate :: app :: rankingsequence :: RankingSequence_UploadPairsMenu_UploadPersonMenuItem,
        god_item: crate::app::rankingsequence::RankingSequence_UploadPairsMenu_UploadGodMenuItem,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_AddUploadPairsMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_AddUploadPairsMenuItemMethods>::ctor(
            this,
            person_item,
            god_item,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_UploadDataMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.UploadDataMenuItem"
)]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_UploadPairsMenu_UploadDataMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_UploadDataMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_UploadDataMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_UploadDataMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_UploadDataMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_PairMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.PairMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct RankingSequence_PairMenuItem {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::nexranking::NexRanking_Data,
    #[static_field]
    #[rename(name = "MaxRatingSlot")]
    pub max_rating_slot: u32,
}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_PairMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::nexranking::NexRanking_Data) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_PairMenuItem {
    pub fn new(data: crate::app::nexranking::NexRanking_Data) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_PairMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_PairMenuItemMethods>::ctor(this, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_UploadGodMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.UploadGodMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct RankingSequence_UploadPairsMenu_UploadGodMenuItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_UploadGodMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_UploadGodMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_UploadGodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_UploadGodMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RankingSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RankingSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RankingSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RankingSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RankingSequence_Label {
    pub fn menu() -> Self {
        Self { value: 0 }
    }

    pub fn ranking() -> Self {
        Self { value: 1 }
    }

    pub fn upload_menu() -> Self {
        Self { value: 2 }
    }

    pub fn upload_data() -> Self {
        Self { value: 3 }
    }

    pub fn upload_pairs_list() -> Self {
        Self { value: 4 }
    }

    pub fn add_upload_pairs() -> Self {
        Self { value: 5 }
    }

    pub fn random_add_upload_pairs() -> Self {
        Self { value: 6 }
    }

    pub fn clear_upload_pairs() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_TopMenu.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.TopMenu")]
#[parent(crate::system::object::Object)]
pub struct RankingSequence_TopMenu {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_TopMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_TopMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_TopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_TopMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.UploadPairsMenu")]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu)]
pub struct RankingSequence_UploadPairsMenu {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_TopMenu_RankingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.TopMenu.RankingMenuItem")]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_TopMenu_RankingMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_TopMenu_RankingMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_TopMenu_RankingMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_TopMenu_RankingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_TopMenu_RankingMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_TopMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.TopMenu.BaseMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct RankingSequence_TopMenu_BaseMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_TopMenu_BaseMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_TopMenu_BaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_TopMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_TopMenu_BaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_UploadPairsMenu_ClearUploadPairsMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RankingSequence.UploadPairsMenu.ClearUploadPairsMenuItem"
)]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_UploadPairsMenu_ClearUploadPairsMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_UploadPairsMenu_ClearUploadPairsMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_UploadPairsMenu_ClearUploadPairsMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_UploadPairsMenu_ClearUploadPairsMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_UploadPairsMenu_ClearUploadPairsMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence_TopMenu_UploadMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence.TopMenu.UploadMenuItem")]
#[parent(crate::app::rankingsequence::RankingSequence_TopMenu_BaseMenuItem)]
pub struct RankingSequence_TopMenu_UploadMenuItem {}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence_TopMenu_UploadMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::rankingsequence::RankingSequence_Label;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence_TopMenu_UploadMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence_TopMenu_UploadMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequence_TopMenu_UploadMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingsequence/RankingSequence.md")))]
#[::unity2::class(namespace = "App", name = "RankingSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: rankingsequence :: RankingSequence >)]
pub struct RankingSequence {
    #[static_field]
    #[rename(name = "MaxRankingCount")]
    pub max_ranking_count: i32,
    #[rename(name = "m_IsLastChapter")]
    pub m_is_last_chapter: bool,
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
    #[rename(name = "m_ChapterIndex")]
    pub m_chapter_index: i32,
    #[rename(name = "m_SelectNetRankingIndex")]
    pub m_select_net_ranking_index: i32,
    #[rename(name = "m_PersonIndex")]
    pub m_person_index: i32,
    #[rename(name = "m_GodIndex")]
    pub m_god_index: i32,
    #[rename(name = "m_RandomAddNum")]
    pub m_random_add_num: i32,
    #[rename(name = "m_Pairs")]
    pub m_pairs: crate::system::collections::generic::list_1::List_1<
        crate::app::nexranking::NexRanking_Data,
    >,
}

#[cfg(feature = "app-rankingsequence")]
#[::unity2::methods]
impl RankingSequence {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetChapter", args = 0)]
    pub fn get_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "Menu", args = 0)]
    pub fn menu(self) -> ();

    #[method(name = "ErrorDialog", args = 0)]
    pub fn error_dialog(self) -> ();

    #[method(name = "DownloadRanking", args = 0)]
    pub fn download_ranking(self) -> ();

    #[method(name = "IsDownloadSuccess", args = 0)]
    pub fn is_download_success(self) -> bool;

    #[method(name = "OpenRankingMenu", args = 0)]
    pub fn open_ranking_menu(self) -> ();

    #[method(name = "CloseRankingMenu", args = 0)]
    pub fn close_ranking_menu(self) -> ();

    #[method(name = "UploadMenu", args = 0)]
    pub fn upload_menu(self) -> ();

    #[method(name = "UploadData", args = 0)]
    pub fn upload_data(self) -> ();

    #[method(name = "UploadPairsList", args = 0)]
    pub fn upload_pairs_list(self) -> ();

    #[method(name = "AddUploadPairs", args = 0)]
    pub fn add_upload_pairs(self) -> ();

    #[method(name = "RandomAddUploadPairs", args = 0)]
    pub fn random_add_upload_pairs(self) -> ();

    #[method(name = "ClearUploadPairs", args = 0)]
    pub fn clear_upload_pairs(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindLastChapter", args = 1)]
    pub fn create_bind_last_chapter(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_last_chapter: bool) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "UnloadRes", args = 0)]
    pub fn unload_res(self) -> ();

    #[method(name = "GetPersonDebugNameByNetRankingIndex", args = 1)]
    pub fn get_person_debug_name_by_net_ranking_index(
        net_ranking_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetGodDebugNameByNetRankingIndex", args = 1)]
    pub fn get_god_debug_name_by_net_ranking_index(
        net_ranking_index: i32,
    ) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-rankingsequence")]
impl RankingSequence {
    pub fn new(is_last_chapter: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingSequenceMethods>::ctor(this, is_last_chapter);
        this
    }
}
