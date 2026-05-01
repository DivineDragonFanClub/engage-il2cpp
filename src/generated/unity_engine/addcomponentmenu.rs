
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addcomponentmenu/AddComponentMenu.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AddComponentMenu")]
pub struct AddComponentMenu {
    #[rename(name = "m_AddComponentMenu")]
    pub m_add_component_menu: ::unity2::Il2CppString,
    #[rename(name = "m_Ordering")]
    pub m_ordering: i32,
}

#[cfg(feature = "unity_engine-addcomponentmenu")]
#[::unity2::methods]
impl AddComponentMenu {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, menu_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, menu_name: ::unity2::Il2CppString, order: i32) -> ();
}

#[cfg(feature = "unity_engine-addcomponentmenu")]
impl AddComponentMenu {
    pub fn new(menu_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddComponentMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAddComponentMenuMethods>::ctor(this, menu_name);
        this
    }

    pub fn new_2(menu_name: ::unity2::Il2CppString, order: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddComponentMenu),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAddComponentMenuMethods>::ctor_2(this, menu_name, order);
        this
    }
}
