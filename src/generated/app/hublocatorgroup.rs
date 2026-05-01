
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hublocatorgroup/HubLocatorGroup.md")))]
#[::unity2::class(namespace = "App", name = "HubLocatorGroup")]
#[parent(crate::system::object::Object)]
pub struct HubLocatorGroup {
    #[rename(name = "m_Active")]
    pub m_active: bool,
    #[rename(name = "m_SystemActive")]
    pub m_system_active: bool,
    #[rename(name = "m_EventActive")]
    pub m_event_active: bool,
    #[rename(name = "m_inactiveObjects")]
    pub m_inactive_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_LoadingCharaCount")]
    pub m_loading_chara_count: i32,
    #[rename(name = "m_hashTable")]
    pub m_hash_table: crate::system::collections::generic::list_1::List_1<i32>,
}

#[cfg(feature = "app-hublocatorgroup")]
#[::unity2::methods]
impl HubLocatorGroup {
    #[method(name = "get_ActiveGroupRoot", args = 0)]
    pub fn get_active_group_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_UnitList", args = 0)]
    pub fn get_unit_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >;

    #[method(name = "get_AccessList", args = 0)]
    pub fn get_access_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::hubaccess::HubAccess>;

    #[method(name = "set_AccessList", args = 1)]
    pub fn set_access_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubaccess::HubAccess,
        >,
    ) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, active: bool) -> ();

    #[method(name = "SetSystemActive", args = 1)]
    pub fn set_system_active(self, system_active: bool) -> ();

    #[method(name = "SetEventActive", args = 1)]
    pub fn set_event_active(self, active: bool) -> ();

    #[method(name = "Contains", args = 2)]
    pub fn contains(
        self,
        param: ::unity2::Array<::unity2::Il2CppString>,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "SetCaseActive", args = 2)]
    pub fn set_case_active(
        self,
        active: bool,
        ignore_node: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "UpdateActive", args = 0)]
    pub fn update_active(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "AddNewLocator", args = 2)]
    pub fn add_new_locator(
        self,
        locator: crate::unity_engine::gameobject::GameObject,
        pid: ::unity2::Il2CppString,
    ) -> crate::app::hubaccess::HubAccess;

    #[method(name = "ClearLocator", args = 1)]
    pub fn clear_locator(self, locator: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "ReloadLocator", args = 1)]
    pub fn reload_locator(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "ReleaseCharacters", args = 0)]
    pub fn release_characters(self) -> ();

    #[method(name = "LocateAccessObjects", args = 0)]
    pub fn locate_access_objects(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "SetupAfter", args = 0)]
    pub fn setup_after(self) -> ();

    #[method(name = "RefreshConfirmContent", args = 1)]
    pub fn refresh_confirm_content(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "RefreshConfirmAll", args = 0)]
    pub fn refresh_confirm_all(self) -> ();

    #[method(name = "FindAccess", args = 1)]
    pub fn find_access(self, pid: ::unity2::Il2CppString) -> crate::app::hubaccess::HubAccess;

    #[method(name = "UpdateLocator", args = 1)]
    pub fn update_locator(self, locator: ::unity2::Il2CppString) -> ();

    #[method(name = "RelocateAccess", args = 0)]
    pub fn relocate_access(self) -> ();

    #[method(name = "IsCharacterLoading", args = 0)]
    pub fn is_character_loading(self) -> bool;

    #[method(name = "SetupCharacterFadeLength", args = 2)]
    pub fn setup_character_fade_length(
        self,
        chara: crate::combat::character::Character,
        fade_distance: f32,
    ) -> ();

    #[method(name = "SetupCharacterFadeRadius", args = 2)]
    pub fn setup_character_fade_radius(
        self,
        chara: crate::combat::character::Character,
        radius: f32,
    ) -> ();

    #[method(name = "GetPlaceWaer", args = 2)]
    pub fn get_place_waer(
        self,
        pid: ::unity2::Il2CppString,
        access: crate::app::hubaccess::HubAccess,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SaveAccessory", args = 0)]
    pub fn save_accessory(self) -> ();

    #[method(name = "RestoreAccessory", args = 0)]
    pub fn restore_accessory(self) -> ();

    #[method(name = "ResetLookAt", args = 0)]
    pub fn reset_look_at(self) -> ();

    #[method(name = "ResetBody", args = 0)]
    pub fn reset_body(self) -> ();

    #[method(name = "Reload", args = 1)]
    pub fn reload(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "IncrementLoadingCount", args = 0)]
    pub fn increment_loading_count(self) -> ();

    #[method(name = "CreateCharacter", args = 4)]
    pub fn create_character(
        self,
        pid: ::unity2::Il2CppString,
        locator: crate::unity_engine::gameobject::GameObject,
        access: crate::app::hubaccess::HubAccess,
        callback: crate::system::action_1::Action_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
    ) -> ();
}

#[cfg(feature = "app-hublocatorgroup")]
impl HubLocatorGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubLocatorGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IHubLocatorGroupMethods>::ctor(this);
        this
    }
}
