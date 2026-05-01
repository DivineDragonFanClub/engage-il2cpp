
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexkillbonus/NexKillBonus_ServerSequence.md")))]
#[::unity2::class(namespace = "App", name = "NexKillBonus.ServerSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct NexKillBonus_ServerSequence {
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::nexkillbonus::NexKillBonus_ServerSequence_Mode,
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
    #[rename(name = "m_KillCountImage")]
    pub m_kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    #[rename(name = "m_KilledCountImage")]
    pub m_killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
}

#[cfg(feature = "app-nexkillbonus")]
#[::unity2::methods]
impl NexKillBonus_ServerSequence {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        mode: crate::app::nexkillbonus::NexKillBonus_ServerSequence_Mode,
        cid: ::unity2::Il2CppString,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> ();

    #[method(name = "Login", args = 0)]
    pub fn login(self) -> ();

    #[method(name = "Postlogin", args = 0)]
    pub fn postlogin(self) -> ();

    #[method(name = "SearchData", args = 0)]
    pub fn search_data(self) -> ();

    #[method(name = "SearchSelfData", args = 0)]
    pub fn search_self_data(self) -> ();

    #[method(name = "DeleteData", args = 0)]
    pub fn delete_data(self) -> ();

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> ();

    #[method(name = "PostData", args = 0)]
    pub fn post_data(self) -> ();

    #[method(name = "UpdateData", args = 0)]
    pub fn update_data(self) -> ();

    #[method(name = "Succeeded", args = 0)]
    pub fn succeeded(self) -> ();

    #[method(name = "SetError", args = 1)]
    pub fn set_error(self, error: crate::app::neterror::NetError_App) -> ();

    #[method(name = "Error", args = 0)]
    pub fn error(self) -> ();

    #[method(name = "OpenWaitMessage", args = 0)]
    pub fn open_wait_message(self) -> ();

    #[method(name = "CloseWaitMessage", args = 0)]
    pub fn close_wait_message(self) -> ();

    #[method(name = "GetDataType", args = 0)]
    pub fn get_data_type(self) -> u16;

    #[method(name = "CreateBindDownload", args = 2)]
    pub fn create_bind_download(
        super_: crate::app::procinst::ProcInst,
        cid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateBindUpload", args = 4)]
    pub fn create_bind_upload(
        super_: crate::app::procinst::ProcInst,
        cid: ::unity2::Il2CppString,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> ();

    #[method(name = "CreateBindImpl", args = 5)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        mode: crate::app::nexkillbonus::NexKillBonus_ServerSequence_Mode,
        cid: ::unity2::Il2CppString,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> ();
}

#[cfg(feature = "app-nexkillbonus")]
impl NexKillBonus_ServerSequence {
    pub fn new(
        mode: crate::app::nexkillbonus::NexKillBonus_ServerSequence_Mode,
        cid: ::unity2::Il2CppString,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexKillBonus_ServerSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INexKillBonus_ServerSequenceMethods>::ctor(
            this,
            mode,
            cid,
            kill_count_image,
            killed_count_image,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexkillbonus/NexKillBonus.md")))]
#[::unity2::class(namespace = "App", name = "NexKillBonus")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: nexkillbonus :: NexKillBonus >)]
pub struct NexKillBonus {
    #[static_field]
    #[rename(name = "MaxWidth")]
    pub max_width: i32,
    #[static_field]
    #[rename(name = "MaxHeight")]
    pub max_height: i32,
    #[static_field]
    #[rename(name = "MaxPostData")]
    pub max_post_data: i32,
    #[static_field]
    #[rename(name = "Period")]
    pub period: i32,
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[static_field]
    #[rename(name = "StreamBufferSize")]
    pub stream_buffer_size: i32,
    #[static_field]
    #[rename(name = "GetBufferSize")]
    pub get_buffer_size: i32,
    #[rename(name = "m_IsSucceeded")]
    pub m_is_succeeded: bool,
    #[rename(name = "m_SerializeStream")]
    pub m_serialize_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_SerializeStreamBuffer")]
    pub m_serialize_stream_buffer: ::unity2::Array<u8>,
}

#[cfg(feature = "app-nexkillbonus")]
#[::unity2::methods]
impl NexKillBonus {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "Download", args = 2)]
    pub fn download(
        self,
        super_: crate::app::procinst::ProcInst,
        cid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Upload", args = 4)]
    pub fn upload(
        self,
        super_: crate::app::procinst::ProcInst,
        cid: ::unity2::Il2CppString,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> bool;

    #[method(name = "ClearSucceeded", args = 0)]
    pub fn clear_succeeded(self) -> ();

    #[method(name = "get_IsSucceeded", args = 0)]
    pub fn get_is_succeeded(self) -> bool;

    #[method(name = "SetData", args = 2)]
    pub fn set_data(self, cid: ::unity2::Il2CppString, data: ::unity2::Array<u8>) -> ();

    #[method(name = "CreateData", args = 4)]
    pub fn create_data(
        self,
        kill_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        killed_count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
        data: ::unity2::Array<u8>,
        data_size: u32,
    ) -> ();

    #[method(name = "AddCounts", args = 2)]
    pub fn add_counts(
        self,
        kind: crate::app::nexkillbonus::NexKillBonus_Kinds,
        count_image: crate::app::mapkillbonus::MapKillBonus_CountImage,
    ) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(self, data: ::unity2::Array<u8>, data_size: u32) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, data: ::unity2::Array<u8>) -> ();

    #[method(name = "ClearData", args = 0)]
    pub fn clear_data(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-nexkillbonus")]
impl NexKillBonus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NexKillBonus),
                ::core::stringify!(new),
            )
        });
        <Self as INexKillBonusMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexkillbonus/NexKillBonus_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexKillBonus_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexKillBonus_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexKillBonus.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexKillBonus_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexKillBonus_Kinds {
    pub fn kill() -> Self {
        Self { value: 0 }
    }

    pub fn killed() -> Self {
        Self { value: 1 }
    }

    pub fn max() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexkillbonus/NexKillBonus_ServerSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexKillBonus_ServerSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexKillBonus_ServerSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexKillBonus.ServerSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexKillBonus_ServerSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexKillBonus_ServerSequence_Label {
    pub fn search_data() -> Self {
        Self { value: 0 }
    }

    pub fn search_self_data() -> Self {
        Self { value: 1 }
    }

    pub fn delete_data() -> Self {
        Self { value: 2 }
    }

    pub fn get_data() -> Self {
        Self { value: 3 }
    }

    pub fn post_data() -> Self {
        Self { value: 4 }
    }

    pub fn update_data() -> Self {
        Self { value: 5 }
    }

    pub fn succeeded() -> Self {
        Self { value: 6 }
    }

    pub fn error() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nexkillbonus/NexKillBonus_ServerSequence_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NexKillBonus_ServerSequence_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NexKillBonus_ServerSequence_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NexKillBonus.ServerSequence.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NexKillBonus_ServerSequence_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NexKillBonus_ServerSequence_Mode {
    pub fn download() -> Self {
        Self { value: 0 }
    }

    pub fn upload() -> Self {
        Self { value: 1 }
    }
}
