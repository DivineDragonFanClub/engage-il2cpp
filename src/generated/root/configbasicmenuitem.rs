
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/root/configbasicmenuitem/ConfigBasicMenuItem.md")))]
#[::unity2::class(namespace = "", name = "ConfigBasicMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ConfigBasicMenuItem {
    #[rename(name = "m_ConfigMethod")]
    pub m_config_method: crate::root::configbasicmenuitem::ConfigBasicMenuItem_ConfigMethodKind,
    #[rename(name = "m_TitleText")]
    pub m_title_text: ::unity2::Il2CppString,
    #[rename(name = "m_CommandText")]
    pub m_command_text: ::unity2::Il2CppString,
    #[rename(name = "m_HelpText")]
    pub m_help_text: ::unity2::Il2CppString,
    #[rename(name = "m_IsArrow")]
    pub m_is_arrow: bool,
    #[rename(name = "m_IsCommandIcon")]
    pub m_is_command_icon: bool,
    #[rename(name = "m_GaugeRatio")]
    pub m_gauge_ratio: f32,
}

#[cfg(feature = "root-configbasicmenuitem")]
#[::unity2::methods]
impl ConfigBasicMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "InitContent", args = 0)]
    pub fn init_content(self) -> ();

    #[method(name = "InitColor", args = 0)]
    pub fn init_color(self) -> ();

    #[method(name = "SetTitleText", args = 1)]
    pub fn set_title_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateText", args = 0)]
    pub fn update_text(self) -> ();

    #[method(name = "IsConfigMethod", args = 1)]
    pub fn is_config_method(
        self,
        config_method: crate::root::configbasicmenuitem::ConfigBasicMenuItem_ConfigMethodKind,
    ) -> bool;

    #[method(name = "ChangeKeyValueImpl", args = 5)]
    pub fn change_key_value_impl(value: f64, min: f64, max: f64, step: f64, is_repeat: bool)
        -> f64;

    #[method(name = "ChangeKeyValue", args = 4)]
    pub fn change_key_value(value: i32, min: i32, max: i32, step: i32) -> i32;

    #[method(name = "ChangeKeyValue", args = 4)]
    pub fn change_key_value_2(value: f32, min: f32, max: f32, step: f32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "root-configbasicmenuitem")]
impl ConfigBasicMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConfigBasicMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IConfigBasicMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/root/configbasicmenuitem/ConfigBasicMenuItem_ConfigMethodKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ConfigBasicMenuItem_ConfigMethodKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ConfigBasicMenuItem_ConfigMethodKind {
    const NAMESPACE: &'static str = "";

    const NAME: &'static str = "ConfigBasicMenuItem.ConfigMethodKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ConfigBasicMenuItem_ConfigMethodKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ConfigBasicMenuItem_ConfigMethodKind {
    pub fn switch() -> Self {
        Self { value: 0 }
    }

    pub fn gauge() -> Self {
        Self { value: 1 }
    }
}
