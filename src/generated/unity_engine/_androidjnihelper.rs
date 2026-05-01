
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/_androidjnihelper/_AndroidJNIHelper.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "_AndroidJNIHelper")]
#[parent(crate::system::object::Object)]
pub struct _AndroidJNIHelper {}

#[cfg(feature = "unity_engine-_androidjnihelper")]
#[::unity2::methods]
impl _AndroidJNIHelper {
    #[method(name = "CreateJavaProxy", args = 2)]
    pub fn create_java_proxy(
        delegate_handle: ::unity2::IntPtr,
        proxy: crate::unity_engine::androidjavaproxy::AndroidJavaProxy,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateJavaRunnable", args = 1)]
    pub fn create_java_runnable(
        jrunnable: crate::unity_engine::androidjavarunnable::AndroidJavaRunnable,
    ) -> ::unity2::IntPtr;

    #[method(name = "InvokeJavaProxyMethod", args = 3)]
    pub fn invoke_java_proxy_method(
        proxy: crate::unity_engine::androidjavaproxy::AndroidJavaProxy,
        jmethod_name: ::unity2::IntPtr,
        jargs: ::unity2::IntPtr,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateJNIArgArray", args = 1)]
    pub fn create_jni_arg_array(
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::Array<crate::unity_engine::jvalue::jvalue>;

    #[method(name = "UnboxArray", args = 1)]
    pub fn unbox_array(
        obj: crate::unity_engine::androidjavaobject::AndroidJavaObject,
    ) -> crate::system::object::Object;

    #[method(name = "Unbox", args = 1)]
    pub fn unbox(
        obj: crate::unity_engine::androidjavaobject::AndroidJavaObject,
    ) -> crate::system::object::Object;

    #[method(name = "Box", args = 1)]
    pub fn r#box(
        obj: crate::system::object::Object,
    ) -> crate::unity_engine::androidjavaobject::AndroidJavaObject;

    #[method(name = "DeleteJNIArgArray", args = 2)]
    pub fn delete_jni_arg_array(
        args: ::unity2::Array<crate::system::object::Object>,
        jni_args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ();

    #[method(name = "ConvertToJNIArray", args = 1)]
    pub fn convert_to_jni_array(array: ::unity2::IlInstance) -> ::unity2::IntPtr;

    #[method(name = "GetConstructorID", args = 2)]
    pub fn get_constructor_id(
        jclass: ::unity2::IntPtr,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetConstructorID", args = 2)]
    pub fn get_constructor_id_2(
        jclass: ::unity2::IntPtr,
        signature: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodID", args = 4)]
    pub fn get_method_id(
        jclass: ::unity2::IntPtr,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
        is_static: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodIDFallback", args = 4)]
    pub fn get_method_id_fallback(
        jclass: ::unity2::IntPtr,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
        is_static: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetSignature", args = 1)]
    pub fn get_signature(obj: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "GetSignature", args = 1)]
    pub fn get_signature_2(
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::Il2CppString;
}
