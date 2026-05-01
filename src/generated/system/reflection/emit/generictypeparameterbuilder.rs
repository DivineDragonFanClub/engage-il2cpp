
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::typeinfo::ITypeInfo;
use crate::system::reflection::typeinfo::TypeInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/emit/generictypeparameterbuilder/GenericTypeParameterBuilder.md")))]
#[::unity2::class(
    namespace = "System.Reflection.Emit",
    name = "GenericTypeParameterBuilder"
)]
#[parent(crate::system::reflection::typeinfo::TypeInfo)]
pub struct GenericTypeParameterBuilder {}

#[cfg(feature = "system-reflection-emit-generictypeparameterbuilder")]
#[::unity2::methods]
impl GenericTypeParameterBuilder {
    #[method(name = "get_Assembly", args = 0)]
    pub fn get_assembly(self) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "get_AssemblyQualifiedName", args = 0)]
    pub fn get_assembly_qualified_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_BaseType", args = 0)]
    pub fn get_base_type(self) -> ::unity2::SystemType;

    #[method(name = "get_FullName", args = 0)]
    pub fn get_full_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Namespace", args = 0)]
    pub fn get_namespace(self) -> ::unity2::Il2CppString;

    #[method(name = "GetElementType", args = 0)]
    pub fn get_element_type(self) -> ::unity2::SystemType;

    #[method(name = "get_UnderlyingSystemType", args = 0)]
    pub fn get_underlying_system_type(self) -> ::unity2::SystemType;

    #[method(name = "GetAttributeFlagsImpl", args = 0)]
    pub fn get_attribute_flags_impl(
        self,
    ) -> crate::system::reflection::typeattributes::TypeAttributes;

    #[method(name = "GetConstructorImpl", args = 5)]
    pub fn get_constructor_impl(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        binder: crate::system::reflection::binder::Binder,
        call_convention: crate::system::reflection::callingconventions::CallingConventions,
        types: ::unity2::Array<::unity2::SystemType>,
        modifiers: ::unity2::Array<crate::system::reflection::parametermodifier::ParameterModifier>,
    ) -> crate::system::reflection::constructorinfo::ConstructorInfo;

    #[method(name = "GetConstructors", args = 1)]
    pub fn get_constructors(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::constructorinfo::ConstructorInfo>;

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

    #[method(name = "GetEvent", args = 2)]
    pub fn get_event(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::system::reflection::eventinfo::EventInfo;

    #[method(name = "GetEvents", args = 1)]
    pub fn get_events(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::eventinfo::EventInfo>;

    #[method(name = "GetField", args = 2)]
    pub fn get_field(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::system::reflection::fieldinfo::FieldInfo;

    #[method(name = "GetFields", args = 1)]
    pub fn get_fields(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::fieldinfo::FieldInfo>;

    #[method(name = "GetInterface", args = 2)]
    pub fn get_interface(
        self,
        name: ::unity2::Il2CppString,
        ignore_case: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetInterfaces", args = 0)]
    pub fn get_interfaces(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetMethodImpl", args = 6)]
    pub fn get_method_impl(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        binder: crate::system::reflection::binder::Binder,
        call_convention: crate::system::reflection::callingconventions::CallingConventions,
        types: ::unity2::Array<::unity2::SystemType>,
        modifiers: ::unity2::Array<crate::system::reflection::parametermodifier::ParameterModifier>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetMethods", args = 1)]
    pub fn get_methods(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

    #[method(name = "GetNestedType", args = 2)]
    pub fn get_nested_type(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::SystemType;

    #[method(name = "GetNestedTypes", args = 1)]
    pub fn get_nested_types(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetProperties", args = 1)]
    pub fn get_properties(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::propertyinfo::PropertyInfo>;

    #[method(name = "GetPropertyImpl", args = 6)]
    pub fn get_property_impl(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        binder: crate::system::reflection::binder::Binder,
        return_type: ::unity2::SystemType,
        types: ::unity2::Array<::unity2::SystemType>,
        modifiers: ::unity2::Array<crate::system::reflection::parametermodifier::ParameterModifier>,
    ) -> crate::system::reflection::propertyinfo::PropertyInfo;

    #[method(name = "HasElementTypeImpl", args = 0)]
    pub fn has_element_type_impl(self) -> bool;

    #[method(name = "IsArrayImpl", args = 0)]
    pub fn is_array_impl(self) -> bool;

    #[method(name = "IsByRefImpl", args = 0)]
    pub fn is_by_ref_impl(self) -> bool;

    #[method(name = "IsCOMObjectImpl", args = 0)]
    pub fn is_com_object_impl(self) -> bool;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "IsPointerImpl", args = 0)]
    pub fn is_pointer_impl(self) -> bool;

    #[method(name = "IsPrimitiveImpl", args = 0)]
    pub fn is_primitive_impl(self) -> bool;
}
