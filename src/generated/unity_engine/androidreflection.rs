
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidreflection/AndroidReflection.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidReflection")]
#[parent(crate::system::object::Object)]
pub struct AndroidReflection {
    #[static_field]
    #[rename(name = "s_ReflectionHelperClass")]
    pub s_reflection_helper_class: crate::unity_engine::globaljavaobjectref::GlobalJavaObjectRef,
    #[static_field]
    #[rename(name = "s_ReflectionHelperGetConstructorID")]
    pub s_reflection_helper_get_constructor_id: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_ReflectionHelperGetMethodID")]
    pub s_reflection_helper_get_method_id: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_ReflectionHelperGetFieldID")]
    pub s_reflection_helper_get_field_id: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_ReflectionHelperGetFieldSignature")]
    pub s_reflection_helper_get_field_signature: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_ReflectionHelperNewProxyInstance")]
    pub s_reflection_helper_new_proxy_instance: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_ReflectionHelperSetNativeExceptionOnProxy")]
    pub s_reflection_helper_set_native_exception_on_proxy: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_FieldGetDeclaringClass")]
    pub s_field_get_declaring_class: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-androidreflection")]
#[::unity2::methods]
impl AndroidReflection {
    #[method(name = "IsPrimitive", args = 1)]
    pub fn is_primitive(t: ::unity2::SystemType) -> bool;

    #[method(name = "IsAssignableFrom", args = 2)]
    pub fn is_assignable_from(t: ::unity2::SystemType, from: ::unity2::SystemType) -> bool;

    #[method(name = "GetStaticMethodID", args = 3)]
    pub fn get_static_method_id(
        clazz: ::unity2::Il2CppString,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodID", args = 3)]
    pub fn get_method_id(
        clazz: ::unity2::Il2CppString,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetConstructorMember", args = 2)]
    pub fn get_constructor_member(
        jclass: ::unity2::IntPtr,
        signature: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodMember", args = 4)]
    pub fn get_method_member(
        jclass: ::unity2::IntPtr,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
        is_static: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "NewProxyInstance", args = 2)]
    pub fn new_proxy_instance(
        delegate_handle: ::unity2::IntPtr,
        interfaze: ::unity2::IntPtr,
    ) -> ::unity2::IntPtr;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
