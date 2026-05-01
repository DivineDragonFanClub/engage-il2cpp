
use crate::app::instanceitem_1::IInstanceItem_1;
use crate::app::instanceitem_1::InstanceItem_1;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::paramitem::IParamItem;
use crate::app::paramitem::ParamItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/boolitem_1/BoolItem_1.md")))]
#[::unity2::class(namespace = "App", name = "BoolItem`1")]
pub struct BoolItem_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-boolitem_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> BoolItem_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, instance: T0) -> ();

    #[method(name = "GetBool", args = 0)]
    pub fn get_bool(self) -> bool;

    #[method(name = "SetBool", args = 1)]
    pub fn set_bool(self, value: bool) -> ();

    #[method(name = "SwapBool", args = 0)]
    pub fn swap_bool(self) -> ();

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();
}

#[cfg(feature = "app-boolitem_1")]
impl<T0: ::unity2::ClassIdentity> BoolItem_1<T0> {
    pub fn new(instance: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoolItem_1),
                ::core::stringify!(new),
            )
        });
        <Self as IBoolItem_1Methods<T0>>::ctor(this, instance);
        this
    }
}
