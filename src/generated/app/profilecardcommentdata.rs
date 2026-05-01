
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentdata/ProfileCardCommentData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: profilecardcommentdata :: ProfileCardCommentData >)]
pub struct ProfileCardCommentData {
    #[static_field]
    #[rename(name = "CategoryMid")]
    pub category_mid: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-profilecardcommentdata")]
#[::unity2::methods]
impl ProfileCardCommentData {
    #[method(name = "get_Id", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(
        self,
    ) -> crate::app::profilecardcommentdata::ProfileCardCommentData_Categories;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(
        self,
        value: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
    ) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> crate::app::profilecardcondition::ProfileCardCondition;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: crate::app::profilecardcondition::ProfileCardCondition)
        -> ();

    #[method(name = "get_Arg", args = 0)]
    pub fn get_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Arg", args = 1)]
    pub fn set_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCategoryMid", args = 1)]
    pub fn get_category_mid(
        category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardcommentdata")]
impl ProfileCardCommentData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentdata/ProfileCardCommentData_Categories.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardCommentData_Categories {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardCommentData_Categories {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardCommentData.Categories";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardCommentData_Categories {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardCommentData_Categories {
    pub fn character_feeling() -> Self {
        Self { value: 0 }
    }

    pub fn feature_position() -> Self {
        Self { value: 1 }
    }

    pub fn adposition_conjection() -> Self {
        Self { value: 2 }
    }

    pub fn tearms() -> Self {
        Self { value: 3 }
    }

    pub fn class_style() -> Self {
        Self { value: 4 }
    }

    pub fn person_name() -> Self {
        Self { value: 5 }
    }

    pub fn num() -> Self {
        Self { value: 6 }
    }
}
