
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/resourcemanager_2/ResourceManager_2.md")))]
#[::unity2::class(namespace = "App", name = "ResourceManager")]
#[parent(crate::system::object::Object)]
pub struct ResourceManager_2 {
# [static_field] # [rename (name = "EntryCount")] pub entry_count : i32 ,
# [static_field] # [rename (name = "GlobalCount")] pub global_count : i32 ,
# [static_field] # [rename (name = "s_LockObject")] pub s_lock_object : :: unity2 :: IlInstance ,
# [static_field] # [rename (name = "s_GlobalHandles")] pub s_global_handles : crate :: system :: collections :: generic :: stack_1 :: Stack_1 < crate :: app :: resourcehandle_2 :: ResourceHandle_2 > ,
# [static_field] # [rename (name = "s_GlobalStacks")] pub s_global_stacks : crate :: system :: collections :: generic :: stack_1 :: Stack_1 < crate :: system :: collections :: generic :: stack_1 :: Stack_1 < crate :: app :: resourcehandle_2 :: ResourceHandle_2 > > ,
# [static_field] # [rename (name = "s_LinkedStack")] pub s_linked_stack : crate :: system :: collections :: generic :: stack_1 :: Stack_1 < crate :: system :: collections :: generic :: linkedlist_1 :: LinkedList_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > > ,
# [static_field] # [rename (name = "s_IsInitialized")] pub s_is_initialized : bool ,
}

#[cfg(feature = "app-resourcemanager_2")]
#[::unity2::methods]
impl ResourceManager_2 {
    #[method(name = "get_s_Globals", args = 0)]
    pub fn get_s_globals() -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::system::collections::generic::stack_1::Stack_1<
            crate::app::resourcehandle_2::ResourceHandle_2,
        >,
    >;

    #[method(name = "get_s_Entries", args = 0)]
    pub fn get_s_entries () -> crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < :: unity2 :: Il2CppString , crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ;

    #[method(name = "get_s_Counters", args = 0)]
    pub fn get_s_counters(
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<::unity2::Il2CppString, i32>;

    #[method(name = "get_s_Releases", args = 0)]
    pub fn get_s_releases () -> crate :: system :: collections :: generic :: queue_1 :: Queue_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ;

    #[method(name = "get_s_Files", args = 0)]
    pub fn get_s_files(
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<::unity2::Il2CppString, bool>;

    #[method(name = "IsInitialized", args = 0)]
    pub fn is_initialized() -> bool;

    #[method(name = "TryAddFile", args = 2)]
    pub fn try_add_file(key: ::unity2::Il2CppString, is_scene: bool) -> bool;

    #[method(name = "EntryFileTable", args = 0)]
    pub fn entry_file_table() -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Release", args = 0)]
    pub fn release() -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        path: ::unity2::Il2CppString,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(path: ::unity2::Il2CppString) -> ();

    #[method(name = "Release", args = 1)]
    pub fn release_2(
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "IsLoading", args = 1)]
    pub fn is_loading_2(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "ReleaseGlobal", args = 1)]
    pub fn release_global(path: ::unity2::Il2CppString) -> ();

    #[method(name = "TryReleaseGlobal", args = 1)]
    pub fn try_release_global(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "Instantiate", args = 2)]
    pub fn instantiate(
        path: ::unity2::Il2CppString,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Instantiate", args = 2)]
    pub fn instantiate_2(
        path: ::unity2::Il2CppString,
        parent: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FileExists", args = 1)]
    pub fn file_exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-resourcemanager_2")]
impl ResourceManager_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceManager_2),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceManager_2Methods>::ctor(this);
        this
    }
}
