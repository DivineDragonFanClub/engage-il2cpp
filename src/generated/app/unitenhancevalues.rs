
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenhancevalues/UnitEnhanceValues.md")))]
#[::unity2::class(namespace = "App", name = "UnitEnhanceValues")]
#[parent(crate::system::object::Object)]
pub struct UnitEnhanceValues {
    #[static_field]
    #[rename(name = "Num")]
    pub num: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Values")]
    pub m_values: ::unity2::Array<i32>,
}

#[cfg(feature = "app-unitenhancevalues")]
#[::unity2::methods]
impl UnitEnhanceValues {
    #[method(name = "IsZero", args = 0)]
    pub fn is_zero(self) -> bool;

    #[method(name = "Set", args = 1)]
    pub fn set(self, values: crate::app::unitenhancevalues::UnitEnhanceValues) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        values: crate::app::unitenhancevalues::UnitEnhanceValues,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "Sub", args = 1)]
    pub fn sub(self, values: crate::app::unitenhancevalues::UnitEnhanceValues) -> ();

    #[method(name = "Merge", args = 2)]
    pub fn merge(
        self,
        values: crate::app::unitenhancevalues::UnitEnhanceValues,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, capability: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(
        self,
        capability: crate::app::capabilitysbyte::CapabilitySbyte,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "Merge", args = 1)]
    pub fn merge_2(self, capability: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_3(
        self,
        capability: crate::app::capabilitysbyte::CapabilitySbyte,
        decay: i32,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_4(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        value: i32,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, value: i32) -> ();

    #[method(name = "Clamp", args = 2)]
    pub fn clamp(self, min: i32, max: i32) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> i32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: i32) -> ();

    #[method(name = "GetNameForDebug", args = 1)]
    pub fn get_name_for_debug(index: i32) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitenhancevalues")]
impl UnitEnhanceValues {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitEnhanceValues),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitEnhanceValuesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenhancevalues/UnitEnhanceValues_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitEnhanceValues_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitEnhanceValues_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitEnhanceValues.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitEnhanceValues_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitEnhanceValues_Type {
    pub fn capability_begin() -> Self {
        Self { value: 0 }
    }

    pub fn capability_end() -> Self {
        Self { value: 11 }
    }

    pub fn num() -> Self {
        Self { value: 11 }
    }
}
