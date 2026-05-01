
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty")]
#[parent(crate::app::debugcommandline::DebugCommandline_Property)]
pub struct DebugCommandline_OptProperty {
    #[rename(name = "m_Attribute")]
    pub m_attribute: crate::app::debugcommandline::DebugCommandline_OptionAttribute,
    #[rename(name = "m_Setter")]
    pub m_setter: crate::app::debugcommandline::DebugCommandline_OptProperty_Setter,
}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        info: crate::system::reflection::propertyinfo::PropertyInfo,
        attribute: crate::app::debugcommandline::DebugCommandline_OptionAttribute,
    ) -> ();

    #[method(name = "Process", args = 3)]
    pub fn process(
        self,
        obj: crate::system::object::Object,
        args: ::unity2::Array<::unity2::Il2CppString>,
        arg_index: i32,
    ) -> i32;

    #[method(name = "get_SortValue", args = 0)]
    pub fn get_sort_value(self) -> i32;
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty {
    pub fn new(
        info: crate::system::reflection::propertyinfo::PropertyInfo,
        attribute: crate::app::debugcommandline::DebugCommandline_OptionAttribute,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptPropertyMethods>::ctor(this, info, attribute);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptwProperty.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptwProperty")]
#[parent(crate::app::debugcommandline::DebugCommandline_Property)]
pub struct DebugCommandline_OptwProperty {
    #[rename(name = "m_Attribute")]
    pub m_attribute: crate::app::debugcommandline::DebugCommandline_OptionWildcardAttribute,
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptwProperty {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        info: crate::system::reflection::propertyinfo::PropertyInfo,
        attribute: crate::app::debugcommandline::DebugCommandline_OptionWildcardAttribute,
    ) -> ();

    #[method(name = "Process", args = 3)]
    pub fn process(
        self,
        obj: crate::system::object::Object,
        args: ::unity2::Array<::unity2::Il2CppString>,
        arg_index: i32,
    ) -> i32;

    #[method(name = "get_SortValue", args = 0)]
    pub fn get_sort_value(self) -> i32;
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptwProperty {
    pub fn new(
        info: crate::system::reflection::propertyinfo::PropertyInfo,
        attribute: crate::app::debugcommandline::DebugCommandline_OptionWildcardAttribute,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptwProperty),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptwPropertyMethods>::ctor(this, info, attribute);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptionWildcardAttribute.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptionWildcardAttribute")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptBaseAttribute)]
pub struct DebugCommandline_OptionWildcardAttribute {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptionWildcardAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, pattern: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptionWildcardAttribute {
    pub fn new(pattern: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptionWildcardAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptionWildcardAttributeMethods>::ctor(this, pattern);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptionAttribute.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptionAttribute")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptBaseAttribute)]
pub struct DebugCommandline_OptionAttribute {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptionAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptionAttribute {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptionAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptionAttributeMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty_BoolSetter.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty.BoolSetter")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptProperty_Setter)]
pub struct DebugCommandline_OptProperty_BoolSetter {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty_BoolSetter {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        obj: crate::system::object::Object,
        property_info: crate::system::reflection::propertyinfo::PropertyInfo,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_IsNeedValue", args = 0)]
    pub fn get_is_need_value(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty_BoolSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty_BoolSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptProperty_BoolSetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty_IntSetter.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty.IntSetter")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptProperty_Setter)]
pub struct DebugCommandline_OptProperty_IntSetter {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty_IntSetter {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        obj: crate::system::object::Object,
        property_info: crate::system::reflection::propertyinfo::PropertyInfo,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_IsNeedValue", args = 0)]
    pub fn get_is_need_value(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty_IntSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty_IntSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptProperty_IntSetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty_FloatSetter.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty.FloatSetter")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptProperty_Setter)]
pub struct DebugCommandline_OptProperty_FloatSetter {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty_FloatSetter {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        obj: crate::system::object::Object,
        property_info: crate::system::reflection::propertyinfo::PropertyInfo,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_IsNeedValue", args = 0)]
    pub fn get_is_need_value(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty_FloatSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty_FloatSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptProperty_FloatSetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty_StringSetter.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty.StringSetter")]
#[parent(crate::app::debugcommandline::DebugCommandline_OptProperty_Setter)]
pub struct DebugCommandline_OptProperty_StringSetter {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty_StringSetter {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        obj: crate::system::object::Object,
        property_info: crate::system::reflection::propertyinfo::PropertyInfo,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_IsNeedValue", args = 0)]
    pub fn get_is_need_value(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty_StringSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty_StringSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptProperty_StringSetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptBaseAttribute.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptBaseAttribute")]
pub struct DebugCommandline_OptBaseAttribute {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptBaseAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptBaseAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptBaseAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptBaseAttributeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline")]
#[parent(crate::system::object::Object)]
pub struct DebugCommandline {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_OptProperty_Setter.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.OptProperty.Setter")]
#[parent(crate::system::object::Object)]
pub struct DebugCommandline_OptProperty_Setter {}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_OptProperty_Setter {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        obj: crate::system::object::Object,
        property_info: crate::system::reflection::propertyinfo::PropertyInfo,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_IsNeedValue", args = 0)]
    pub fn get_is_need_value(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_OptProperty_Setter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_OptProperty_Setter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_OptProperty_SetterMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandline/DebugCommandline_Property.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandline.Property")]
#[parent(crate::system::object::Object)]
pub struct DebugCommandline_Property {
    #[static_field]
    #[rename(name = "OptSortValue")]
    pub opt_sort_value: i32,
    #[static_field]
    #[rename(name = "OptwSortValue")]
    pub optw_sort_value: i32,
    #[rename(name = "m_Info")]
    pub m_info: crate::system::reflection::propertyinfo::PropertyInfo,
}

#[cfg(feature = "app-debugcommandline")]
#[::unity2::methods]
impl DebugCommandline_Property {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, info: crate::system::reflection::propertyinfo::PropertyInfo) -> ();

    #[method(name = "Process", args = 3)]
    pub fn process(
        self,
        obj: crate::system::object::Object,
        args: ::unity2::Array<::unity2::Il2CppString>,
        arg_index: i32,
    ) -> i32;

    #[method(name = "get_SortValue", args = 0)]
    pub fn get_sort_value(self) -> i32;
}

#[cfg(feature = "app-debugcommandline")]
impl DebugCommandline_Property {
    pub fn new(info: crate::system::reflection::propertyinfo::PropertyInfo) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandline_Property),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandline_PropertyMethods>::ctor(this, info);
        this
    }
}
