
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialdata/TutorialData_Types.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialData_Types {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialData_Types {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialData.Types";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialData_Types {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialData_Types {
    pub fn battle() -> Self {
        Self { value: 0 }
    }

    pub fn god() -> Self {
        Self { value: 1 }
    }

    pub fn hub() -> Self {
        Self { value: 2 }
    }

    pub fn kizuna_gmap() -> Self {
        Self { value: 3 }
    }

    pub fn challenge() -> Self {
        Self { value: 4 }
    }

    pub fn pick_up() -> Self {
        Self { value: 5 }
    }

    pub fn none() -> Self {
        Self { value: 6 }
    }

    pub fn num() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialdata/TutorialData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialData_Flags {
    pub fn lock() -> Self {
        Self { value: 0 }
    }

    pub fn unlock() -> Self {
        Self { value: 1 }
    }

    pub fn read() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialdata/TutorialData_Notices.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialData_Notices {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialData_Notices {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialData.Notices";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialData_Notices {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialData_Notices {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn silent() -> Self {
        Self { value: 1 }
    }

    pub fn forced() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialdata/TutorialData_SSTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialData_SSTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialData_SSTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialData.SSTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialData_SSTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialData_SSTypes {
    pub fn common() -> Self {
        Self { value: 0 }
    }

    pub fn by_language() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialdata/TutorialData.md")))]
#[::unity2::class(namespace = "App", name = "TutorialData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: tutorialdata :: TutorialData >)]
pub struct TutorialData {
    #[static_field]
    #[rename(name = "FirstIndex")]
    pub first_index: i32,
}

#[cfg(feature = "app-tutorialdata")]
#[::unity2::methods]
impl TutorialData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "TryGetFromTutid", args = 1)]
    pub fn try_get_from_tutid(
        tutid: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::tutorialdata::TutorialData>;

    #[method(name = "LockPickUpTutorial", args = 0)]
    pub fn lock_pick_up_tutorial() -> ();

    #[method(name = "SetListFlag", args = 1)]
    pub fn set_list_flag(flag: crate::app::tutorialdata::TutorialData_Flags) -> ();

    #[method(name = "GetListFromType", args = 2)]
    pub fn get_list_from_type(
        r#type: crate::app::tutorialdata::TutorialData_Types,
        is_get_all_tutorial: bool,
    ) -> crate::app::structarraylist_1::StructArrayList_1<crate::app::tutorialdata::TutorialData>;

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Title", args = 0)]
    pub fn get_title(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Title", args = 1)]
    pub fn set_title(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SpriteAtlas", args = 0)]
    pub fn get_sprite_atlas(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SpriteAtlas", args = 1)]
    pub fn set_sprite_atlas(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::app::tutorialdata::TutorialData_Types;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: crate::app::tutorialdata::TutorialData_Types) -> ();

    #[method(name = "get_Notice", args = 0)]
    pub fn get_notice(self) -> crate::app::tutorialdata::TutorialData_Notices;

    #[method(name = "set_Notice", args = 1)]
    pub fn set_notice(self, value: crate::app::tutorialdata::TutorialData_Notices) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_No", args = 0)]
    pub fn get_no(self) -> i32;

    #[method(name = "set_No", args = 1)]
    pub fn set_no(self, value: i32) -> ();

    #[method(name = "get_SSType", args = 0)]
    pub fn get_ss_type(self) -> crate::app::tutorialdata::TutorialData_SSTypes;

    #[method(name = "set_SSType", args = 1)]
    pub fn set_ss_type(self, value: crate::app::tutorialdata::TutorialData_SSTypes) -> ();

    #[method(name = "GetGlobalFlagsName", args = 1)]
    pub fn get_global_flags_name(tutid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "SetGlobalFlags", args = 2)]
    pub fn set_global_flags(
        tutid: ::unity2::Il2CppString,
        flag: crate::app::tutorialdata::TutorialData_Flags,
    ) -> ();

    #[method(name = "GetGlobalFlags", args = 1)]
    pub fn get_global_flags(
        tutid: ::unity2::Il2CppString,
    ) -> crate::app::tutorialdata::TutorialData_Flags;

    #[method(name = "IsGlobalFlags", args = 2)]
    pub fn is_global_flags(
        tutid: ::unity2::Il2CppString,
        flag: crate::app::tutorialdata::TutorialData_Flags,
    ) -> bool;

    #[method(name = "IsLock", args = 1)]
    pub fn is_lock(tutid: ::unity2::Il2CppString) -> bool;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "CompletedChapter", args = 1)]
    pub fn completed_chapter(chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-tutorialdata")]
impl TutorialData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TutorialData),
                ::core::stringify!(new),
            )
        });
        <Self as ITutorialDataMethods>::ctor(this);
        this
    }
}
