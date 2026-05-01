
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::stringitem::IStringItem;
use crate::app::stringitem::StringItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/funcitem/FuncItem_Func.md")))]
#[::unity2::class(namespace = "App", name = "FuncItem.Func")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct FuncItem_Func {}

#[cfg(feature = "app-funcitem")]
#[::unity2::methods]
impl FuncItem_Func {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        item: crate::app::menuitem::MenuItem,
    ) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-funcitem")]
impl FuncItem_Func {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FuncItem_Func),
                ::core::stringify!(new),
            )
        });
        <Self as IFuncItem_FuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/funcitem/FuncItem.md")))]
#[::unity2::class(namespace = "App", name = "FuncItem")]
#[parent(crate::app::stringitem::StringItem)]
pub struct FuncItem {
    #[rename(name = "m_Func")]
    pub m_func: crate::app::funcitem::FuncItem_Func,
}

#[cfg(feature = "app-funcitem")]
#[::unity2::methods]
impl FuncItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        func: crate::app::funcitem::FuncItem_Func,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-funcitem")]
impl FuncItem {
    pub fn new(name: ::unity2::Il2CppString, func: crate::app::funcitem::FuncItem_Func) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FuncItem),
                ::core::stringify!(new),
            )
        });
        <Self as IFuncItemMethods>::ctor(this, name, func);
        this
    }
}
