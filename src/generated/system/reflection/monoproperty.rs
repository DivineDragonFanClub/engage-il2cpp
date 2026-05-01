
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::propertyinfo::IPropertyInfo;
use crate::system::reflection::propertyinfo::PropertyInfo;
use crate::system::reflection::runtimepropertyinfo::IRuntimePropertyInfo;
use crate::system::reflection::runtimepropertyinfo::RuntimePropertyInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoproperty/MonoProperty_Getter_2.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoProperty.Getter`2")]
pub struct MonoProperty_Getter_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-reflection-monoproperty")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MonoProperty_Getter_2<T0, T1> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, target_0: T0) -> T1;
}

#[cfg(feature = "system-reflection-monoproperty")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MonoProperty_Getter_2<T0, T1> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoProperty_Getter_2),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoProperty_Getter_2Methods<T0, T1>>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoproperty/MonoProperty.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoProperty")]
#[parent(crate::system::reflection::runtimepropertyinfo::RuntimePropertyInfo)]
pub struct MonoProperty {
    #[rename(name = "klass")]
    pub klass: ::unity2::IntPtr,
    #[rename(name = "prop")]
    pub prop: ::unity2::IntPtr,
    #[rename(name = "info")]
    pub info: crate::system::reflection::monopropertyinfo::MonoPropertyInfo,
    #[rename(name = "cached")]
    pub cached: crate::system::reflection::pinfo::PInfo,
    #[rename(name = "cached_getter")]
    pub cached_getter: crate::system::reflection::monoproperty::MonoProperty_GetterAdapter,
}

#[cfg(feature = "system-reflection-monoproperty")]
#[::unity2::methods]
impl MonoProperty {
    #[method(name = "CachePropertyInfo", args = 1)]
    pub fn cache_property_info(self, flags: crate::system::reflection::pinfo::PInfo) -> ();

    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(
        self,
    ) -> crate::system::reflection::propertyattributes::PropertyAttributes;

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "get_PropertyType", args = 0)]
    pub fn get_property_type(self) -> ::unity2::SystemType;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAccessors", args = 1)]
    pub fn get_accessors(
        self,
        non_public: bool,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

    #[method(name = "GetGetMethod", args = 1)]
    pub fn get_get_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetIndexParameters", args = 0)]
    pub fn get_index_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetSetMethod", args = 1)]
    pub fn get_set_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetConstantValue", args = 0)]
    pub fn get_constant_value(self) -> crate::system::object::Object;

    #[method(name = "GetRawConstantValue", args = 0)]
    pub fn get_raw_constant_value(self) -> crate::system::object::Object;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes(
        self,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes_2(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "CreateGetterDelegate", args = 1)]
    pub fn create_getter_delegate(
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::system::reflection::monoproperty::MonoProperty_GetterAdapter;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        obj: crate::system::object::Object,
        index: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "GetOptionalCustomModifiers", args = 0)]
    pub fn get_optional_custom_modifiers(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetRequiredCustomModifiers", args = 0)]
    pub fn get_required_custom_modifiers(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monoproperty")]
impl MonoProperty {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoProperty),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoPropertyMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoproperty/MonoProperty_StaticGetter_1.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoProperty.StaticGetter`1")]
pub struct MonoProperty_StaticGetter_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-reflection-monoproperty")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MonoProperty_StaticGetter_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> T0;
}

#[cfg(feature = "system-reflection-monoproperty")]
impl<T0: ::unity2::ClassIdentity> MonoProperty_StaticGetter_1<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoProperty_StaticGetter_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoProperty_StaticGetter_1Methods<T0>>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoproperty/MonoProperty_GetterAdapter.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoProperty.GetterAdapter")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MonoProperty_GetterAdapter {}

#[cfg(feature = "system-reflection-monoproperty")]
#[::unity2::methods]
impl MonoProperty_GetterAdapter {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, target_0: crate::system::object::Object) -> crate::system::object::Object;
}

#[cfg(feature = "system-reflection-monoproperty")]
impl MonoProperty_GetterAdapter {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoProperty_GetterAdapter),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoProperty_GetterAdapterMethods>::ctor(this, object, method);
        this
    }
}
