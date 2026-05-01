
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tipsdata/TipsData.md")))]
#[::unity2::class(namespace = "App", name = "TipsData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: tipsdata :: TipsData >)]
pub struct TipsData {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::tipsdata::TipsData_Kinds,
}

#[cfg(feature = "app-tipsdata")]
#[::unity2::methods]
impl TipsData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Title", args = 0)]
    pub fn get_title(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Title", args = 1)]
    pub fn set_title(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Tips", args = 0)]
    pub fn get_tips(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tips", args = 1)]
    pub fn set_tips(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_OwnID", args = 0)]
    pub fn get_own_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_OwnID", args = 1)]
    pub fn set_own_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IconInfoID", args = 0)]
    pub fn get_icon_info_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IconInfoID", args = 1)]
    pub fn set_icon_info_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Variable", args = 0)]
    pub fn get_variable(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Variable", args = 1)]
    pub fn set_variable(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Allow", args = 0)]
    pub fn get_allow(self) -> crate::app::tipsdata::TipsData_Allows;

    #[method(name = "set_Allow", args = 1)]
    pub fn set_allow(self, value: crate::app::tipsdata::TipsData_Allows) -> ();

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::tipsdata::TipsData_Kinds;

    #[method(name = "GetTipsID", args = 0)]
    pub fn get_tips_id(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "IsCondition", args = 0)]
    pub fn is_condition(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-tipsdata")]
impl TipsData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TipsData),
                ::core::stringify!(new),
            )
        });
        <Self as ITipsDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tipsdata/TipsData_Allows.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TipsData_Allows {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TipsData_Allows {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TipsData.Allows";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TipsData_Allows {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TipsData_Allows {
    pub fn map() -> Self {
        Self { value: 1 }
    }

    pub fn hub() -> Self {
        Self { value: 2 }
    }

    pub fn gmap() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tipsdata/TipsData_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TipsData_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TipsData_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TipsData.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TipsData_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TipsData_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn item() -> Self {
        Self { value: 1 }
    }

    pub fn skill() -> Self {
        Self { value: 2 }
    }
}
