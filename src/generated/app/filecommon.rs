
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/filecommon/FileCommon.md")))]
#[::unity2::class(namespace = "App", name = "FileCommon")]
#[parent(crate::system::object::Object)]
pub struct FileCommon {
    #[static_field]
    #[rename(name = "s_LockObject")]
    pub s_lock_object: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "s_Dictionary")]
    pub s_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::filedata::FileData,
    >,
    #[static_field]
    #[rename(name = "s_PreAsyncList")]
    pub s_pre_async_list:
        crate::system::collections::generic::list_1::List_1<crate::app::filedata::FileData>,
    #[static_field]
    #[rename(name = "s_PostAsyncList")]
    pub s_post_async_list:
        crate::system::collections::generic::list_1::List_1<crate::app::filedata::FileData>,
}

#[cfg(feature = "app-filecommon")]
#[::unity2::methods]
impl FileCommon {
    #[method(name = "ReadAllBytes", args = 1)]
    pub fn read_all_bytes(path: ::unity2::Il2CppString) -> ::unity2::Array<u8>;

    #[method(name = "GetFullPath", args = 1)]
    pub fn get_full_path(path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-filecommon")]
impl FileCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FileCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IFileCommonMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/filecommon/FileCommon_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FileCommon_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FileCommon_State {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FileCommon.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FileCommon_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FileCommon_State {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn r#async() -> Self {
        Self { value: 1 }
    }

    pub fn done() -> Self {
        Self { value: 2 }
    }
}
