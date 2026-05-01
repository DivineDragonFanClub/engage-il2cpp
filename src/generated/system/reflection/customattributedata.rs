
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/customattributedata/CustomAttributeData.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "CustomAttributeData")]
#[parent(crate::system::object::Object)]
pub struct CustomAttributeData {
    #[rename(name = "ctorInfo")]
    pub ctor_info: crate::system::reflection::constructorinfo::ConstructorInfo,
    #[rename(name = "ctorArgs")]
    pub ctor_args: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributetypedargument::CustomAttributeTypedArgument,
    >,
    #[rename(name = "namedArgs")]
    pub named_args: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributenamedargument::CustomAttributeNamedArgument,
    >,
    #[rename(name = "lazyData")]
    pub lazy_data:
        crate::system::reflection::customattributedata::CustomAttributeData_LazyCAttrData,
}

#[cfg(feature = "system-reflection-customattributedata")]
#[::unity2::methods]
impl CustomAttributeData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        ctor_info: crate::system::reflection::constructorinfo::ConstructorInfo,
        assembly: crate::system::reflection::assembly::Assembly,
        data: ::unity2::IntPtr,
        data_length: u32,
    ) -> ();

    #[method(name = "ResolveArgumentsInternal", args = 6)]
    pub fn resolve_arguments_internal(
        ctor: crate::system::reflection::constructorinfo::ConstructorInfo,
        assembly: crate::system::reflection::assembly::Assembly,
        data: ::unity2::IntPtr,
        data_length: u32,
        ctor_args: ::unity2::Array<crate::system::object::Object>,
        named_args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "ResolveArguments", args = 0)]
    pub fn resolve_arguments(self) -> ();

    #[method(name = "get_Constructor", args = 0)]
    pub fn get_constructor(self) -> crate::system::reflection::constructorinfo::ConstructorInfo;

    #[method(name = "get_ConstructorArguments", args = 0)]
    pub fn get_constructor_arguments(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributetypedargument::CustomAttributeTypedArgument,
    >;

    #[method(name = "get_NamedArguments", args = 0)]
    pub fn get_named_arguments(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributenamedargument::CustomAttributeNamedArgument,
    >;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes(
        target: crate::system::reflection::assembly::Assembly,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes_2(
        target: crate::system::reflection::memberinfo::MemberInfo,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "GetCustomAttributesInternal", args = 1)]
    pub fn get_custom_attributes_internal(
        target: crate::system::runtimetype::RuntimeType,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes_3(
        target: crate::system::reflection::module::Module,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes_4(
        target: crate::system::reflection::parameterinfo::ParameterInfo,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "get_AttributeType", args = 0)]
    pub fn get_attribute_type(self) -> ::unity2::SystemType;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "system-reflection-customattributedata")]
impl CustomAttributeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomAttributeData),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomAttributeDataMethods>::ctor(this);
        this
    }

    pub fn new_2(
        ctor_info: crate::system::reflection::constructorinfo::ConstructorInfo,
        assembly: crate::system::reflection::assembly::Assembly,
        data: ::unity2::IntPtr,
        data_length: u32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomAttributeData),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICustomAttributeDataMethods>::ctor_2(this, ctor_info, assembly, data, data_length);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/customattributedata/CustomAttributeData_LazyCAttrData.md")))]
#[::unity2::class(
    namespace = "System.Reflection",
    name = "CustomAttributeData.LazyCAttrData"
)]
#[parent(crate::system::object::Object)]
pub struct CustomAttributeData_LazyCAttrData {
    #[rename(name = "assembly")]
    pub assembly: crate::system::reflection::assembly::Assembly,
    #[rename(name = "data")]
    pub data: ::unity2::IntPtr,
    #[rename(name = "data_length")]
    pub data_length: u32,
}

#[cfg(feature = "system-reflection-customattributedata")]
#[::unity2::methods]
impl CustomAttributeData_LazyCAttrData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-customattributedata")]
impl CustomAttributeData_LazyCAttrData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomAttributeData_LazyCAttrData),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomAttributeData_LazyCAttrDataMethods>::ctor(this);
        this
    }
}
