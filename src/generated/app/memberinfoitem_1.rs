
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/memberinfoitem_1/MemberInfoItem_1.md")))]
#[::unity2::class(namespace = "App", name = "MemberInfoItem`1")]
pub struct MemberInfoItem_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Object")]
    pub m_object: ::unity2::IlInstance,
    #[rename(name = "m_Info")]
    pub m_info: T0,
}

#[cfg(feature = "app-memberinfoitem_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MemberInfoItem_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, obj: crate::system::object::Object, info: T0) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnAlign1", args = 0)]
    pub fn get_column_align1(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: crate::system::object::Object) -> ();

    #[method(name = "GetValueType", args = 0)]
    pub fn get_value_type(self) -> ::unity2::SystemType;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();
}

#[cfg(feature = "app-memberinfoitem_1")]
impl<T0: ::unity2::ClassIdentity> MemberInfoItem_1<T0> {
    pub fn new(obj: crate::system::object::Object, info: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemberInfoItem_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMemberInfoItem_1Methods<T0>>::ctor(this, obj, info);
        this
    }
}
