
use crate::app::instanceitem_1::IInstanceItem_1;
use crate::app::instanceitem_1::InstanceItem_1;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::paramitem::IParamItem;
use crate::app::paramitem::ParamItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/floatitem_1/FloatItem_1.md")))]
#[::unity2::class(namespace = "App", name = "FloatItem`1")]
pub struct FloatItem_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-floatitem_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> FloatItem_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, instance: T0) -> ();

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: f32) -> ();

    #[method(name = "GetStep", args = 0)]
    pub fn get_step(self) -> f32;

    #[method(name = "GetMin", args = 0)]
    pub fn get_min(self) -> f32;

    #[method(name = "GetMax", args = 0)]
    pub fn get_max(self) -> f32;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();
}

#[cfg(feature = "app-floatitem_1")]
impl<T0: ::unity2::ClassIdentity> FloatItem_1<T0> {
    pub fn new(instance: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FloatItem_1),
                ::core::stringify!(new),
            )
        });
        <Self as IFloatItem_1Methods<T0>>::ctor(this, instance);
        this
    }
}
