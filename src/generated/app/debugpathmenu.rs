
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_ShowError.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.ShowError")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_ShowError {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_ShowError {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        message: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_ShowError {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_ShowError),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_ShowErrorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebugPathMenu_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebugPathMenu_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DebugPathMenu.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugPathMenu_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebugPathMenu_Result {
    pub fn selected() -> Self {
        Self { value: 0 }
    }

    pub fn canceled() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_PCDriveMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.PCDriveMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_PCDriveMenu {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_PCDriveMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_PCDriveMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_PCDriveMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_PCDriveMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Directory.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.Directory")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_Directory {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_Directory {
    #[method(name = "EnumEntries", args = 2)]
    pub fn enum_entries(
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        path: ::unity2::Il2CppString,
    ) -> crate::app::debugpathmenu::DebugPathMenu_Directory_EnumResult;

    #[method(name = "IsExists", args = 1)]
    pub fn is_exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "EnumEntriesNX", args = 2)]
    pub fn enum_entries_nx(
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        path: ::unity2::Il2CppString,
    ) -> crate::app::debugpathmenu::DebugPathMenu_Directory_EnumResult;

    #[method(name = "MakeEntryDataNX", args = 2)]
    pub fn make_entry_data_nx(
        is_directory: bool,
        entry_name: ::unity2::Il2CppString,
    ) -> crate::app::debugpathmenu::DebugPathMenu_EntryData;

    #[method(name = "IsExistsNX", args = 1)]
    pub fn is_exists_nx(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "EnumEntriesWin", args = 2)]
    pub fn enum_entries_win(
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        path: ::unity2::Il2CppString,
    ) -> crate::app::debugpathmenu::DebugPathMenu_Directory_EnumResult;

    #[method(name = "MakeEntryDataWin", args = 2)]
    pub fn make_entry_data_win(
        is_directory: bool,
        full_path: ::unity2::Il2CppString,
    ) -> crate::app::debugpathmenu::DebugPathMenu_EntryData;

    #[method(name = "IsExistsWin", args = 1)]
    pub fn is_exists_win(path: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_Directory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_Directory),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_DirectoryMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Path.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.Path")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_Path {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_Path {
    #[method(name = "NormalizeForDirectory", args = 1)]
    pub fn normalize_for_directory(original_path: ::unity2::Il2CppString)
        -> ::unity2::Il2CppString;

    #[method(name = "NormalizeForFile", args = 1)]
    pub fn normalize_for_file(original_path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "NormalizePathSeparator", args = 1)]
    pub fn normalize_path_separator(
        original_path: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetParentDirectory", args = 1)]
    pub fn get_parent_directory(original_path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetDirectoryName", args = 1)]
    pub fn get_directory_name(original_path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetFileName", args = 1)]
    pub fn get_file_name(original_path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_Path {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_Path),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_PathMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Categories.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebugPathMenu_Categories {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebugPathMenu_Categories {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DebugPathMenu.Categories";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugPathMenu_Categories {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebugPathMenu_Categories {
    pub fn pc() -> Self {
        Self { value: 1 }
    }

    pub fn assets() -> Self {
        Self { value: 2 }
    }

    pub fn all() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_ShowError_ErrorMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.ShowError.ErrorMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugPathMenu_ShowError_ErrorMenuItem {
    #[rename(name = "m_Error")]
    pub m_error: ::unity2::Il2CppString,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_ShowError_ErrorMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, error: ::unity2::Il2CppString) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_ShowError_ErrorMenuItem {
    pub fn new(error: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_ShowError_ErrorMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_ShowError_ErrorMenuItemMethods>::ctor(this, error);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Setting.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.Setting")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_Setting {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_Setting {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Categories", args = 0)]
    pub fn get_categories(self) -> crate::app::debugpathmenu::DebugPathMenu_Categories;

    #[method(name = "set_Categories", args = 1)]
    pub fn set_categories(self, value: crate::app::debugpathmenu::DebugPathMenu_Categories) -> ();

    #[method(name = "get_IsDirectoryOnly", args = 0)]
    pub fn get_is_directory_only(self) -> bool;

    #[method(name = "set_IsDirectoryOnly", args = 1)]
    pub fn set_is_directory_only(self, value: bool) -> ();

    #[method(name = "get_InitialDirectoryPath", args = 0)]
    pub fn get_initial_directory_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_InitialDirectoryPath", args = 1)]
    pub fn set_initial_directory_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_OnSelect", args = 0)]
    pub fn get_on_select(self) -> crate::system::action_1::Action_1<::unity2::Il2CppString>;

    #[method(name = "set_OnSelect", args = 1)]
    pub fn set_on_select(
        self,
        value: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "get_OnCancel", args = 0)]
    pub fn get_on_cancel(self) -> crate::system::action::Action;

    #[method(name = "set_OnCancel", args = 1)]
    pub fn set_on_cancel(self, value: crate::system::action::Action) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_Setting {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_Setting),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_SettingMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu")]
#[parent(crate::app::procinst::ProcInst)]
pub struct DebugPathMenu {
    #[rename(name = "m_Setting")]
    pub m_setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
    ) -> ();

    #[method(name = "get_LastResult", args = 0)]
    pub fn get_last_result() -> crate::app::debugpathmenu::DebugPathMenu_Result;

    #[method(name = "set_LastResult", args = 1)]
    pub fn set_last_result(value: crate::app::debugpathmenu::DebugPathMenu_Result) -> ();

    #[method(name = "get_LastPath", args = 0)]
    pub fn get_last_path() -> ::unity2::Il2CppString;

    #[method(name = "set_LastPath", args = 1)]
    pub fn set_last_path(value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> ();

    #[method(name = "CreateMenu", args = 0)]
    pub fn create_menu(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu {
    pub fn new(setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenuMethods>::ctor(this, setting);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_Directory_EnumResult.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.Directory.EnumResult")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_Directory_EnumResult {
    #[rename(name = "Entries")]
    pub entries: crate::system::collections::generic::list_1::List_1<
        crate::app::debugpathmenu::DebugPathMenu_EntryData,
    >,
    #[rename(name = "ErrorMessage")]
    pub error_message: ::unity2::Il2CppString,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_Directory_EnumResult {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        entries: crate::system::collections::generic::list_1::List_1<
            crate::app::debugpathmenu::DebugPathMenu_EntryData,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, error_message: ::unity2::Il2CppString) -> ();

    #[method(name = "IsFailure", args = 0)]
    pub fn is_failure(self) -> bool;
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_Directory_EnumResult {
    pub fn new(
        entries: crate::system::collections::generic::list_1::List_1<
            crate::app::debugpathmenu::DebugPathMenu_EntryData,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_Directory_EnumResult),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_Directory_EnumResultMethods>::ctor(this, entries);
        this
    }

    pub fn new_2(error_message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_Directory_EnumResult),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugPathMenu_Directory_EnumResultMethods>::ctor_2(this, error_message);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_CategoriesMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.CategoriesMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_CategoriesMenu {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_CategoriesMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_CategoriesMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_CategoriesMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_CategoriesMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_EntryMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.EntryMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugPathMenu_EntryMenuItem {
    #[static_field]
    #[rename(name = "MinWidth")]
    pub min_width: f32,
    #[rename(name = "m_Setting")]
    pub m_setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
    #[rename(name = "m_CurrentPath")]
    pub m_current_path: ::unity2::Il2CppString,
    #[rename(name = "m_EntryData")]
    pub m_entry_data: crate::app::debugpathmenu::DebugPathMenu_EntryData,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_EntryMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        current_path: ::unity2::Il2CppString,
        entry_data: crate::app::debugpathmenu::DebugPathMenu_EntryData,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelp", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "MoveToParent", args = 0)]
    pub fn move_to_parent(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "MoveToSub", args = 0)]
    pub fn move_to_sub(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_EntryMenuItem {
    pub fn new(
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        current_path: ::unity2::Il2CppString,
        entry_data: crate::app::debugpathmenu::DebugPathMenu_EntryData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_EntryMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_EntryMenuItemMethods>::ctor(
            this,
            setting,
            current_path,
            entry_data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_CategoriesMenu_PCMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.CategoriesMenu.PCMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugPathMenu_CategoriesMenu_PCMenuItem {
    #[rename(name = "m_Setting")]
    pub m_setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_CategoriesMenu_PCMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_CategoriesMenu_PCMenuItem {
    pub fn new(setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_CategoriesMenu_PCMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_CategoriesMenu_PCMenuItemMethods>::ctor(this, setting);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_EntryData.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.EntryData")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_EntryData {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_EntryData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, is_directory: bool, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsDirectory", args = 0)]
    pub fn get_is_directory(self) -> bool;

    #[method(name = "set_IsDirectory", args = 1)]
    pub fn set_is_directory(self, value: bool) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_EntryData {
    pub fn new(is_directory: bool, name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_EntryData),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_EntryDataMethods>::ctor(this, is_directory, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_CategoriesMenu_AssetsMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DebugPathMenu.CategoriesMenu.AssetsMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugPathMenu_CategoriesMenu_AssetsMenuItem {
    #[rename(name = "m_Setting")]
    pub m_setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_CategoriesMenu_AssetsMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_CategoriesMenu_AssetsMenuItem {
    pub fn new(setting: crate::app::debugpathmenu::DebugPathMenu_Setting) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_CategoriesMenu_AssetsMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_CategoriesMenu_AssetsMenuItemMethods>::ctor(this, setting);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugpathmenu/DebugPathMenu_EntryMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugPathMenu.EntryMenu")]
#[parent(crate::system::object::Object)]
pub struct DebugPathMenu_EntryMenu {}

#[cfg(feature = "app-debugpathmenu")]
#[::unity2::methods]
impl DebugPathMenu_EntryMenu {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        setting: crate::app::debugpathmenu::DebugPathMenu_Setting,
        parent_path: ::unity2::Il2CppString,
        is_root: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugpathmenu")]
impl DebugPathMenu_EntryMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugPathMenu_EntryMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugPathMenu_EntryMenuMethods>::ctor(this);
        this
    }
}
