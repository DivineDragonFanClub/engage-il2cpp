
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menumanager/MenuManager.md")))]
#[::unity2::class(namespace = "App", name = "MenuManager")]
#[parent(crate::system::object::Object)]
pub struct MenuManager {
    #[static_field]
    #[rename(name = "m_menuList")]
    pub m_menu_list:
        crate::system::collections::generic::list_1::List_1<crate::app::basicmenu::BasicMenu>,
}

#[cfg(feature = "app-menumanager")]
#[::unity2::methods]
impl MenuManager {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "PostLoadPrefabAsnc", args = 0)]
    pub fn post_load_prefab_asnc() -> ();

    #[method(name = "AddMenu", args = 1)]
    pub fn add_menu(menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "RemoveMenu", args = 1)]
    pub fn remove_menu(menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all() -> ();

    #[method(name = "DeleteAll", args = 0)]
    pub fn delete_all() -> ();

    #[method(name = "DebugLog_List", args = 0)]
    pub fn debug_log_list() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-menumanager")]
impl MenuManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MenuManager),
                ::core::stringify!(new),
            )
        });
        <Self as IMenuManagerMethods>::ctor(this);
        this
    }
}
