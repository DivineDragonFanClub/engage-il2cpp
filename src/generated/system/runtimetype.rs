
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::typeinfo::ITypeInfo;
use crate::system::reflection::typeinfo::TypeInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/runtimetype/RuntimeType.md")))]
#[::unity2::class(namespace = "System", name = "RuntimeType")]
#[parent(crate::system::reflection::typeinfo::TypeInfo)]
pub struct RuntimeType {
    #[static_field]
    #[rename(name = "ValueType")]
    pub value_type: crate::system::runtimetype::RuntimeType,
    #[static_field]
    #[rename(name = "EnumType")]
    pub enum_type: crate::system::runtimetype::RuntimeType,
    #[static_field]
    #[rename(name = "ObjectType")]
    pub object_type: crate::system::runtimetype::RuntimeType,
    #[static_field]
    #[rename(name = "StringType")]
    pub string_type: crate::system::runtimetype::RuntimeType,
    #[static_field]
    #[rename(name = "DelegateType")]
    pub delegate_type: crate::system::runtimetype::RuntimeType,
    #[static_field]
    #[rename(name = "s_SICtorParamTypes")]
    pub s_si_ctor_param_types: ::unity2::Array<::unity2::SystemType>,
    #[static_field]
    #[rename(name = "MemberBindingMask")]
    pub member_binding_mask: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "InvocationMask")]
    pub invocation_mask: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderNonCreateInstance")]
    pub binder_non_create_instance: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderGetSetProperty")]
    pub binder_get_set_property: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderSetInvokeProperty")]
    pub binder_set_invoke_property: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderGetSetField")]
    pub binder_get_set_field: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderSetInvokeField")]
    pub binder_set_invoke_field: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "BinderNonFieldGetSet")]
    pub binder_non_field_get_set: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "ClassicBindingMask")]
    pub classic_binding_mask: crate::system::reflection::bindingflags::BindingFlags,
    #[static_field]
    #[rename(name = "s_typedRef")]
    pub s_typed_ref: crate::system::runtimetype::RuntimeType,
    #[rename(name = "GenericCache")]
    pub generic_cache: ::unity2::IlInstance,
    #[rename(name = "m_serializationCtor")]
    pub m_serialization_ctor:
        crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo,
}

#[cfg(feature = "system-runtimetype")]
#[::unity2::methods]
impl RuntimeType {
    #[method(name = "ThrowIfTypeNeverValidGenericArgument", args = 1)]
    pub fn throw_if_type_never_valid_generic_argument(
        r#type: crate::system::runtimetype::RuntimeType,
    ) -> ();

    #[method(name = "SanityCheckGenericArguments", args = 2)]
    pub fn sanity_check_generic_arguments(
        generic_arguments: ::unity2::Array<crate::system::runtimetype::RuntimeType>,
        generic_paramters: ::unity2::Array<crate::system::runtimetype::RuntimeType>,
    ) -> ();

    #[method(name = "SplitName", args = 3)]
    pub fn split_name(
        fullname: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        ns: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FilterApplyPrefixLookup", args = 3)]
    pub fn filter_apply_prefix_lookup(
        member_info: crate::system::reflection::memberinfo::MemberInfo,
        name: ::unity2::Il2CppString,
        ignore_case: bool,
    ) -> bool;

    #[method(name = "FilterApplyBase", args = 7)]
    pub fn filter_apply_base(
        member_info: crate::system::reflection::memberinfo::MemberInfo,
        binding_flags: crate::system::reflection::bindingflags::BindingFlags,
        is_public: bool,
        is_non_protected_internal: bool,
        is_static: bool,
        name: ::unity2::Il2CppString,
        prefix_lookup: bool,
    ) -> bool;

    #[method(name = "FilterApplyType", args = 5)]
    pub fn filter_apply_type(
        r#type: ::unity2::SystemType,
        binding_flags: crate::system::reflection::bindingflags::BindingFlags,
        name: ::unity2::Il2CppString,
        prefix_lookup: bool,
        ns: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "FilterApplyMethodInfo", args = 4)]
    pub fn filter_apply_method_info(
        method: crate::system::reflection::runtimemethodinfo::RuntimeMethodInfo,
        binding_flags: crate::system::reflection::bindingflags::BindingFlags,
        call_conv: crate::system::reflection::callingconventions::CallingConventions,
        argument_types: ::unity2::Array<::unity2::SystemType>,
    ) -> bool;

    #[method(name = "FilterApplyConstructorInfo", args = 4)]
    pub fn filter_apply_constructor_info(
        constructor: crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo,
        binding_flags: crate::system::reflection::bindingflags::BindingFlags,
        call_conv: crate::system::reflection::callingconventions::CallingConventions,
        argument_types: ::unity2::Array<::unity2::SystemType>,
    ) -> bool;

    #[method(name = "FilterApplyMethodBase", args = 5)]
    pub fn filter_apply_method_base(
        method_base: crate::system::reflection::methodbase::MethodBase,
        method_flags: crate::system::reflection::bindingflags::BindingFlags,
        binding_flags: crate::system::reflection::bindingflags::BindingFlags,
        call_conv: crate::system::reflection::callingconventions::CallingConventions,
        argument_types: ::unity2::Array<::unity2::SystemType>,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsSpecialSerializableType", args = 0)]
    pub fn is_special_serializable_type(self) -> bool;

    #[method(name = "GetMethods", args = 1)]
    pub fn get_methods(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

    #[method(name = "GetConstructors", args = 1)]
    pub fn get_constructors(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::constructorinfo::ConstructorInfo>;

    #[method(name = "GetProperties", args = 1)]
    pub fn get_properties(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::propertyinfo::PropertyInfo>;

    #[method(name = "GetEvents", args = 1)]
    pub fn get_events(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::eventinfo::EventInfo>;

    #[method(name = "GetFields", args = 1)]
    pub fn get_fields(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::fieldinfo::FieldInfo>;

    #[method(name = "GetNestedTypes", args = 1)]
    pub fn get_nested_types(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetMethodImpl", args = 6)]
    pub fn get_method_impl(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        binder: crate::system::reflection::binder::Binder,
        call_conv: crate::system::reflection::callingconventions::CallingConventions,
        types: ::unity2::Array<::unity2::SystemType>,
        modifiers: ::unity2::Array<crate::system::reflection::parametermodifier::ParameterModifier>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetConstructorImpl", args = 5)]
    pub fn get_constructor_impl(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        binder: crate::system::reflection::binder::Binder,
        call_convention: crate::system::reflection::callingconventions::CallingConventions,
        types: ::unity2::Array<::unity2::SystemType>,
        modifiers: ::unity2::Array<crate::system::reflection::parametermodifier::ParameterModifier>,
    ) -> crate::system::reflection::constructorinfo::ConstructorInfo;

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

    #[method(name = "GetEvent", args = 2)]
    pub fn get_event(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::system::reflection::eventinfo::EventInfo;

    #[method(name = "GetField", args = 2)]
    pub fn get_field(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::system::reflection::fieldinfo::FieldInfo;

    #[method(name = "GetInterface", args = 2)]
    pub fn get_interface(
        self,
        fullname: ::unity2::Il2CppString,
        ignore_case: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetNestedType", args = 2)]
    pub fn get_nested_type(
        self,
        fullname: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::SystemType;

    #[method(name = "GetMember", args = 3)]
    pub fn get_member(
        self,
        name: ::unity2::Il2CppString,
        r#type: crate::system::reflection::membertypes::MemberTypes,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::reflection::memberinfo::MemberInfo>;

    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "GetRuntimeModule", args = 0)]
    pub fn get_runtime_module(self) -> crate::system::reflection::runtimemodule::RuntimeModule;

    #[method(name = "get_Assembly", args = 0)]
    pub fn get_assembly(self) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "GetRuntimeAssembly", args = 0)]
    pub fn get_runtime_assembly(
        self,
    ) -> crate::system::reflection::runtimeassembly::RuntimeAssembly;

    #[method(name = "IsInstanceOfType", args = 1)]
    pub fn is_instance_of_type(self, o: crate::system::object::Object) -> bool;

    #[method(name = "IsSubclassOf", args = 1)]
    pub fn is_subclass_of(self, r#type: ::unity2::SystemType) -> bool;

    #[method(name = "IsAssignableFrom", args = 1)]
    pub fn is_assignable_from(self, c: ::unity2::SystemType) -> bool;

    #[method(name = "IsEquivalentTo", args = 1)]
    pub fn is_equivalent_to(self, other: ::unity2::SystemType) -> bool;

    #[method(name = "get_BaseType", args = 0)]
    pub fn get_base_type(self) -> ::unity2::SystemType;

    #[method(name = "get_UnderlyingSystemType", args = 0)]
    pub fn get_underlying_system_type(self) -> ::unity2::SystemType;

    #[method(name = "GetAttributeFlagsImpl", args = 0)]
    pub fn get_attribute_flags_impl(
        self,
    ) -> crate::system::reflection::typeattributes::TypeAttributes;

    #[method(name = "IsContextfulImpl", args = 0)]
    pub fn is_contextful_impl(self) -> bool;

    #[method(name = "IsByRefImpl", args = 0)]
    pub fn is_by_ref_impl(self) -> bool;

    #[method(name = "IsPrimitiveImpl", args = 0)]
    pub fn is_primitive_impl(self) -> bool;

    #[method(name = "IsPointerImpl", args = 0)]
    pub fn is_pointer_impl(self) -> bool;

    #[method(name = "IsCOMObjectImpl", args = 0)]
    pub fn is_com_object_impl(self) -> bool;

    #[method(name = "IsValueTypeImpl", args = 0)]
    pub fn is_value_type_impl(self) -> bool;

    #[method(name = "get_IsEnum", args = 0)]
    pub fn get_is_enum(self) -> bool;

    #[method(name = "HasElementTypeImpl", args = 0)]
    pub fn has_element_type_impl(self) -> bool;

    #[method(name = "get_GenericParameterAttributes", args = 0)]
    pub fn get_generic_parameter_attributes(
        self,
    ) -> crate::system::reflection::genericparameterattributes::GenericParameterAttributes;

    #[method(name = "get_IsSzArray", args = 0)]
    pub fn get_is_sz_array(self) -> bool;

    #[method(name = "IsArrayImpl", args = 0)]
    pub fn is_array_impl(self) -> bool;

    #[method(name = "GetArrayRank", args = 0)]
    pub fn get_array_rank(self) -> i32;

    #[method(name = "GetElementType", args = 0)]
    pub fn get_element_type(self) -> ::unity2::SystemType;

    #[method(name = "GetEnumNames", args = 0)]
    pub fn get_enum_names(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetEnumValues", args = 0)]
    pub fn get_enum_values(self) -> ::unity2::IlInstance;

    #[method(name = "GetEnumUnderlyingType", args = 0)]
    pub fn get_enum_underlying_type(self) -> ::unity2::SystemType;

    #[method(name = "IsEnumDefined", args = 1)]
    pub fn is_enum_defined(self, value: crate::system::object::Object) -> bool;

    #[method(name = "GetEnumName", args = 1)]
    pub fn get_enum_name(self, value: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "GetGenericArgumentsInternal", args = 0)]
    pub fn get_generic_arguments_internal(
        self,
    ) -> ::unity2::Array<crate::system::runtimetype::RuntimeType>;

    #[method(name = "GetGenericArguments", args = 0)]
    pub fn get_generic_arguments(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "MakeGenericType", args = 1)]
    pub fn make_generic_type(
        self,
        instantiation: ::unity2::Array<::unity2::SystemType>,
    ) -> ::unity2::SystemType;

    #[method(name = "get_IsGenericTypeDefinition", args = 0)]
    pub fn get_is_generic_type_definition(self) -> bool;

    #[method(name = "get_IsGenericParameter", args = 0)]
    pub fn get_is_generic_parameter(self) -> bool;

    #[method(name = "get_GenericParameterPosition", args = 0)]
    pub fn get_generic_parameter_position(self) -> i32;

    #[method(name = "GetGenericTypeDefinition", args = 0)]
    pub fn get_generic_type_definition(self) -> ::unity2::SystemType;

    #[method(name = "get_IsGenericType", args = 0)]
    pub fn get_is_generic_type(self) -> bool;

    #[method(name = "get_IsConstructedGenericType", args = 0)]
    pub fn get_is_constructed_generic_type(self) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::runtimetype::RuntimeType,
        right: crate::system::runtimetype::RuntimeType,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::runtimetype::RuntimeType,
        right: crate::system::runtimetype::RuntimeType,
    ) -> bool;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

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

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "FormatTypeName", args = 1)]
    pub fn format_type_name(self, serialization: bool) -> ::unity2::Il2CppString;

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_MetadataToken", args = 0)]
    pub fn get_metadata_token(self) -> i32;

    #[method(name = "CreateInstanceCheckThis", args = 0)]
    pub fn create_instance_check_this(self) -> ();

    #[method(name = "GetDefaultConstructor", args = 0)]
    pub fn get_default_constructor(self) -> crate::system::reflection::monocmethod::MonoCMethod;

    #[method(name = "GetDefaultMemberName", args = 0)]
    pub fn get_default_member_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSerializationCtor", args = 0)]
    pub fn get_serialization_ctor(
        self,
    ) -> crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo;

    #[method(name = "CreateInstanceMono", args = 1)]
    pub fn create_instance_mono(self, non_public: bool) -> crate::system::object::Object;

    #[method(name = "TryConvertToType", args = 2)]
    pub fn try_convert_to_type(
        self,
        value: crate::system::object::Object,
        failed: bool,
    ) -> crate::system::object::Object;

    #[method(name = "IsConvertibleToPrimitiveType", args = 2)]
    pub fn is_convertible_to_primitive_type(
        value: crate::system::object::Object,
        target_type: ::unity2::SystemType,
    ) -> crate::system::object::Object;

    #[method(name = "make_array_type", args = 1)]
    pub fn make_array_type(self, rank: i32) -> ::unity2::SystemType;

    #[method(name = "MakeArrayType", args = 0)]
    pub fn make_array_type_2(self) -> ::unity2::SystemType;

    #[method(name = "make_byref_type", args = 0)]
    pub fn make_byref_type(self) -> ::unity2::SystemType;

    #[method(name = "MakeByRefType", args = 0)]
    pub fn make_by_ref_type(self) -> ::unity2::SystemType;

    #[method(name = "MakePointerType", args = 1)]
    pub fn make_pointer_type(r#type: ::unity2::SystemType) -> ::unity2::SystemType;

    #[method(name = "MakePointerType", args = 0)]
    pub fn make_pointer_type_2(self) -> ::unity2::SystemType;

    #[method(name = "get_ContainsGenericParameters", args = 0)]
    pub fn get_contains_generic_parameters(self) -> bool;

    #[method(name = "GetGenericParameterConstraints", args = 0)]
    pub fn get_generic_parameter_constraints(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "CreateInstanceForAnotherGenericParameter", args = 2)]
    pub fn create_instance_for_another_generic_parameter(
        generic_type: ::unity2::SystemType,
        generic_argument: crate::system::runtimetype::RuntimeType,
    ) -> crate::system::object::Object;

    #[method(name = "MakeGenericType", args = 2)]
    pub fn make_generic_type_2(
        gt: ::unity2::SystemType,
        types: ::unity2::Array<::unity2::SystemType>,
    ) -> ::unity2::SystemType;

    #[method(name = "GetMethodsByName_native", args = 3)]
    pub fn get_methods_by_name_native(
        self,
        name_ptr: ::unity2::IntPtr,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        ignore_case: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodsByName", args = 4)]
    pub fn get_methods_by_name(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        ignore_case: bool,
        reflected_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<crate::system::reflection::runtimemethodinfo::RuntimeMethodInfo>;

    #[method(name = "GetPropertiesByName_native", args = 3)]
    pub fn get_properties_by_name_native(
        self,
        name: ::unity2::IntPtr,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        icase: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetConstructors_native", args = 1)]
    pub fn get_constructors_native(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetConstructors_internal", args = 2)]
    pub fn get_constructors_internal(
        self,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        reflected_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo>;

    #[method(name = "GetPropertiesByName", args = 4)]
    pub fn get_properties_by_name(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        icase: bool,
        reflected_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<crate::system::reflection::runtimepropertyinfo::RuntimePropertyInfo>;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "IsGenericCOMObjectImpl", args = 0)]
    pub fn is_generic_com_object_impl(self) -> bool;

    #[method(name = "CreateInstanceInternal", args = 1)]
    pub fn create_instance_internal(r#type: ::unity2::SystemType) -> crate::system::object::Object;

    #[method(name = "get_DeclaringMethod", args = 0)]
    pub fn get_declaring_method(self) -> crate::system::reflection::methodbase::MethodBase;

    #[method(name = "getFullName", args = 2)]
    pub fn get_full_name(self, full_name: bool, assembly_qualified: bool)
        -> ::unity2::Il2CppString;

    #[method(name = "GetGenericArgumentsInternal", args = 1)]
    pub fn get_generic_arguments_internal_2(
        self,
        runtime_array: bool,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetEvents_native", args = 2)]
    pub fn get_events_native(
        self,
        name: ::unity2::IntPtr,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetFields_native", args = 2)]
    pub fn get_fields_native(
        self,
        name: ::unity2::IntPtr,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetFields_internal", args = 3)]
    pub fn get_fields_internal(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        reflected_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<crate::system::reflection::runtimefieldinfo::RuntimeFieldInfo>;

    #[method(name = "GetEvents_internal", args = 3)]
    pub fn get_events_internal(
        self,
        name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
        reflected_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<crate::system::reflection::runtimeeventinfo::RuntimeEventInfo>;

    #[method(name = "GetInterfaces", args = 0)]
    pub fn get_interfaces(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetNestedTypes_native", args = 2)]
    pub fn get_nested_types_native(
        self,
        name: ::unity2::IntPtr,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetNestedTypes_internal", args = 2)]
    pub fn get_nested_types_internal(
        self,
        display_name: ::unity2::Il2CppString,
        binding_attr: crate::system::reflection::bindingflags::BindingFlags,
    ) -> ::unity2::Array<crate::system::runtimetype::RuntimeType>;

    #[method(name = "get_AssemblyQualifiedName", args = 0)]
    pub fn get_assembly_qualified_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Namespace", args = 0)]
    pub fn get_namespace(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "get_FullName", args = 0)]
    pub fn get_full_name_2(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-runtimetype")]
impl RuntimeType {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeType),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeTypeMethods>::ctor(this);
        this
    }
}
