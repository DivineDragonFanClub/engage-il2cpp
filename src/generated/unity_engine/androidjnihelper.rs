
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjnihelper/AndroidJNIHelper.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJNIHelper")]
#[parent(crate::system::object::Object)]
pub struct AndroidJNIHelper {}

#[cfg(feature = "unity_engine-androidjnihelper")]
#[::unity2::methods]
impl AndroidJNIHelper {
    #[method(name = "GetConstructorID", args = 2)]
    pub fn get_constructor_id(
        java_class: ::unity2::IntPtr,
        signature: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetMethodID", args = 4)]
    pub fn get_method_id(
        java_class: ::unity2::IntPtr,
        method_name: ::unity2::Il2CppString,
        signature: ::unity2::Il2CppString,
        is_static: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateJavaRunnable", args = 1)]
    pub fn create_java_runnable(
        jrunnable: crate::unity_engine::androidjavarunnable::AndroidJavaRunnable,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateJavaProxy", args = 1)]
    pub fn create_java_proxy(
        proxy: crate::unity_engine::androidjavaproxy::AndroidJavaProxy,
    ) -> ::unity2::IntPtr;

    #[method(name = "CreateJNIArgArray", args = 1)]
    pub fn create_jni_arg_array(
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::Array<crate::unity_engine::jvalue::jvalue>;

    #[method(name = "DeleteJNIArgArray", args = 2)]
    pub fn delete_jni_arg_array(
        args: ::unity2::Array<crate::system::object::Object>,
        jni_args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ();

    #[method(name = "GetConstructorID", args = 2)]
    pub fn get_constructor_id_2(
        jclass: ::unity2::IntPtr,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::IntPtr;
}
