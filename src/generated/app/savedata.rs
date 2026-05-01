
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedata/SaveData_Manager_Task.md")))]
#[::unity2::class(namespace = "App", name = "SaveData.Manager.Task")]
#[parent(crate::system::object::Object)]
pub struct SaveData_Manager_Task {}

#[cfg(feature = "app-savedata")]
#[::unity2::methods]
impl SaveData_Manager_Task {
    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::savedata::SaveData_Manager_TaskKind;

    #[method(name = "set_Kind", args = 1)]
    pub fn set_kind(self, value: crate::app::savedata::SaveData_Manager_TaskKind) -> ();

    #[method(name = "get_Path", args = 0)]
    pub fn get_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Path", args = 1)]
    pub fn set_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "set_Data", args = 1)]
    pub fn set_data(self, value: ::unity2::Array<u8>) -> ();

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i64;

    #[method(name = "set_Size", args = 1)]
    pub fn set_size(self, value: i64) -> ();

    #[method(name = "get_Offset", args = 0)]
    pub fn get_offset(self) -> i64;

    #[method(name = "set_Offset", args = 1)]
    pub fn set_offset(self, value: i64) -> ();

    #[method(name = "get_IsEnableResize", args = 0)]
    pub fn get_is_enable_resize(self) -> bool;

    #[method(name = "set_IsEnableResize", args = 1)]
    pub fn set_is_enable_resize(self, value: bool) -> ();

    #[method(name = "get_Handle", args = 0)]
    pub fn get_handle(self) -> crate::app::savedatahandle::SaveDataHandle;

    #[method(name = "set_Handle", args = 1)]
    pub fn set_handle(self, value: crate::app::savedatahandle::SaveDataHandle) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-savedata")]
impl SaveData_Manager_Task {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SaveData_Manager_Task),
                ::core::stringify!(new),
            )
        });
        <Self as ISaveData_Manager_TaskMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedata/SaveData.md")))]
#[::unity2::class(namespace = "App", name = "SaveData")]
#[parent(crate::system::object::Object)]
pub struct SaveData {
    #[static_field]
    #[rename(name = "MountName")]
    pub mount_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "MountNameWithColon")]
    pub mount_name_with_colon: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_IsMounted")]
    pub s_is_mounted: bool,
}

#[cfg(feature = "app-savedata")]
#[::unity2::methods]
impl SaveData {
    #[method(name = "Setup", args = 0)]
    pub fn setup() -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup() -> ();

    #[method(name = "PauseExitApp", args = 0)]
    pub fn pause_exit_app() -> ();

    #[method(name = "ResumeExitApp", args = 0)]
    pub fn resume_exit_app() -> ();

    #[method(name = "ReadAsync", args = 4)]
    pub fn read_async(
        path: ::unity2::Il2CppString,
        offset: i64,
        data: ::unity2::Array<u8>,
        size: i64,
    ) -> crate::app::savedatareadhandle::SaveDataReadHandle;

    #[method(name = "WriteAsync", args = 5)]
    pub fn write_async(
        path: ::unity2::Il2CppString,
        offset: i64,
        data: ::unity2::Array<u8>,
        size: i64,
        is_enable_resize: bool,
    ) -> crate::app::savedatahandle::SaveDataHandle;

    #[method(name = "DeleteAsync", args = 1)]
    pub fn delete_async(path: ::unity2::Il2CppString)
        -> crate::app::savedatahandle::SaveDataHandle;

    #[method(name = "Commit", args = 1)]
    pub fn commit(mount_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CommitAsync", args = 1)]
    pub fn commit_async(
        mount_name: ::unity2::Il2CppString,
    ) -> crate::app::savedatahandle::SaveDataHandle;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetFileSize", args = 1)]
    pub fn get_file_size(path: ::unity2::Il2CppString) -> i64;

    #[method(name = "Dump", args = 1)]
    pub fn dump(root_path: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsMounted", args = 0)]
    pub fn get_is_mounted() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-savedata")]
impl SaveData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SaveData),
                ::core::stringify!(new),
            )
        });
        <Self as ISaveDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedata/SaveData_Manager_TaskKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SaveData_Manager_TaskKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SaveData_Manager_TaskKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SaveData.Manager.TaskKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SaveData_Manager_TaskKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SaveData_Manager_TaskKind {
    pub fn read() -> Self {
        Self { value: 0 }
    }

    pub fn write() -> Self {
        Self { value: 1 }
    }

    pub fn delete() -> Self {
        Self { value: 2 }
    }

    pub fn commit() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedata/SaveData_Manager_EventKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SaveData_Manager_EventKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SaveData_Manager_EventKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SaveData.Manager.EventKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SaveData_Manager_EventKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SaveData_Manager_EventKind {
    pub fn cleanup() -> Self {
        Self { value: 0 }
    }

    pub fn task() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/savedata/SaveData_Manager.md")))]
#[::unity2::class(namespace = "App", name = "SaveData.Manager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: savedata :: SaveData_Manager >)]
pub struct SaveData_Manager {
    #[rename(name = "m_Tasks")]
    pub m_tasks: crate::system::collections::generic::queue_1::Queue_1<
        crate::app::savedata::SaveData_Manager_Task,
    >,
}

#[cfg(feature = "app-savedata")]
#[::unity2::methods]
impl SaveData_Manager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "RegisterTask", args = 1)]
    pub fn register_task(self, task: crate::app::savedata::SaveData_Manager_Task) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, mount_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(self, path: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetFileSize", args = 1)]
    pub fn get_file_size(self, path: ::unity2::Il2CppString) -> i64;

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, root_path: ::unity2::Il2CppString) -> ();

    #[method(name = "ThreadFunc", args = 0)]
    pub fn thread_func(self) -> ();

    #[method(name = "Read", args = 1)]
    pub fn read(self, task: crate::app::savedata::SaveData_Manager_Task) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, task: crate::app::savedata::SaveData_Manager_Task) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, task: crate::app::savedata::SaveData_Manager_Task) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit_2(self, task: crate::app::savedata::SaveData_Manager_Task) -> ();

    #[method(name = "CommitNX", args = 1)]
    pub fn commit_nx(self, mount_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsExistNX", args = 1)]
    pub fn is_exist_nx(self, path: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetFileSizeNX", args = 1)]
    pub fn get_file_size_nx(self, path: ::unity2::Il2CppString) -> i64;

    #[method(name = "DumpNX", args = 1)]
    pub fn dump_nx(self, root_path: ::unity2::Il2CppString) -> ();

    #[method(name = "Backslash2Slash", args = 1)]
    pub fn backslash2_slash(self, path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-savedata")]
impl SaveData_Manager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SaveData_Manager),
                ::core::stringify!(new),
            )
        });
        <Self as ISaveData_ManagerMethods>::ctor(this);
        this
    }
}
