
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexprofile/NexProfile_ServerSequence_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexProfile_ServerSequence_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexProfile_ServerSequence_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexProfile.ServerSequence.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexProfile_ServerSequence_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexProfile_ServerSequence_Mode {
    pub fn upload() -> Self {
        Self { value: 0 }
    }

    pub fn download() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexprofile/NexProfile.md")))]
#[::unity2::class(namespace = "App", name = "NexProfile")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: nexprofile :: NexProfile >)]
pub struct NexProfile {
    #[rename(name = "m_LastResult")]
    pub m_last_result: crate::app::nexprofile::NexProfile_Results,
    #[rename(name = "m_LastResultData")]
    pub m_last_result_data: crate::app::profilecard::ProfileCard,
}

#[cfg(feature = "app-nexprofile")]
#[::unity2::methods]
impl NexProfile {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Upload", args = 2)]
    pub fn upload(
        self,
        super_: crate::app::procinst::ProcInst,
        profile: crate::app::profilecard::ProfileCard,
    ) -> bool;

    #[method(name = "Download", args = 2)]
    pub fn download(self, super_: crate::app::procinst::ProcInst, principal_id: u64) -> bool;

    #[method(name = "get_Result", args = 0)]
    pub fn get_result(self) -> crate::app::nexprofile::NexProfile_Results;

    #[method(name = "get_ResultData", args = 0)]
    pub fn get_result_data(self) -> crate::app::profilecard::ProfileCard;

    #[method(name = "ClearResult", args = 0)]
    pub fn clear_result(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-nexprofile")]
impl NexProfile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexProfile),
                ::core::stringify!(new),
            )
        });
        <Self as INexProfileMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexprofile/NexProfile_Results.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexProfile_Results {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexProfile_Results {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexProfile.Results";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexProfile_Results {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexProfile_Results {
    pub fn failed() -> Self {
        Self { value: 0 }
    }

    pub fn failed_not_found() -> Self {
        Self { value: 1 }
    }

    pub fn failed_permission_denied() -> Self {
        Self { value: 2 }
    }

    pub fn cancelled() -> Self {
        Self { value: 3 }
    }

    pub fn succeeded() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexprofile/NexProfile_ServerSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexProfile_ServerSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexProfile_ServerSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexProfile.ServerSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexProfile_ServerSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexProfile_ServerSequence_Label {
    pub fn upload() -> Self {
        Self { value: 0 }
    }

    pub fn upload_new() -> Self {
        Self { value: 1 }
    }

    pub fn upload_change() -> Self {
        Self { value: 2 }
    }

    pub fn upload_change_meta() -> Self {
        Self { value: 3 }
    }

    pub fn download() -> Self {
        Self { value: 4 }
    }

    pub fn succeeded() -> Self {
        Self { value: 5 }
    }

    pub fn error() -> Self {
        Self { value: 6 }
    }

    pub fn end() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexprofile/NexProfile_ServerSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexProfile.ServerSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NexProfile_ServerSequence {
    #[static_field]
    #[rename(name = "BufferSizeMax")]
    pub buffer_size_max: i32,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::nexprofile::NexProfile_ServerSequence_Mode,
    #[rename(name = "m_PrincipalID")]
    pub m_principal_id: u64,
    #[rename(name = "m_Profile")]
    pub m_profile: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_DataID")]
    pub m_data_id: u64,
}

#[cfg(feature = "app-nexprofile")]
#[::unity2::methods]
impl NexProfile_ServerSequence {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        mode: crate::app::nexprofile::NexProfile_ServerSequence_Mode,
        principal_id: u64,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();

    #[method(name = "Login", args = 0)]
    pub fn login(self) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "UploadBranch", args = 0)]
    pub fn upload_branch(self) -> ();

    #[method(name = "UploadNew", args = 0)]
    pub fn upload_new(self) -> ();

    #[method(name = "Change", args = 0)]
    pub fn change(self) -> ();

    #[method(name = "ChangeMeta", args = 0)]
    pub fn change_meta(self) -> ();

    #[method(name = "SerializeData", args = 2)]
    pub fn serialize_data(self, data: ::unity2::Array<u8>, size: u32) -> ();

    #[method(name = "Download", args = 0)]
    pub fn download(self) -> ();

    #[method(name = "DeserializeData", args = 1)]
    pub fn deserialize_data(
        self,
        data: ::unity2::Array<u8>,
    ) -> crate::app::profilecard::ProfileCard;

    #[method(name = "Succeeded", args = 0)]
    pub fn succeeded(self) -> ();

    #[method(name = "SetError", args = 1)]
    pub fn set_error(self, error: crate::app::neterror::NetError_App) -> ();

    #[method(name = "SetErrorNotFound", args = 0)]
    pub fn set_error_not_found(self) -> ();

    #[method(name = "SetErrorPermissionDenied", args = 0)]
    pub fn set_error_permission_denied(self) -> ();

    #[method(name = "Error", args = 0)]
    pub fn error(self) -> ();

    #[method(name = "CreateBindDownload", args = 2)]
    pub fn create_bind_download(super_: crate::app::procinst::ProcInst, principal_id: u64) -> ();

    #[method(name = "CreateBindUpload", args = 2)]
    pub fn create_bind_upload(
        super_: crate::app::procinst::ProcInst,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();

    #[method(name = "CreateBindImpl", args = 4)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        mode: crate::app::nexprofile::NexProfile_ServerSequence_Mode,
        principal_id: u64,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();
}

#[cfg(feature = "app-nexprofile")]
impl NexProfile_ServerSequence {
    pub fn new(
        mode: crate::app::nexprofile::NexProfile_ServerSequence_Mode,
        principal_id: u64,
        profile: crate::app::profilecard::ProfileCard,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexProfile_ServerSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexProfile_ServerSequenceMethods>::ctor(this, mode, principal_id, profile);
        this
    }
}
