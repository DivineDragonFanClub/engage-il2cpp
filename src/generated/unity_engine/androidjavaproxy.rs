
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjavaproxy/AndroidJavaProxy.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJavaProxy")]
#[parent(crate::system::object::Object)]
pub struct AndroidJavaProxy {
    #[rename(name = "javaInterface")]
    pub java_interface: crate::unity_engine::androidjavaclass::AndroidJavaClass,
    #[rename(name = "proxyObject")]
    pub proxy_object: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_JavaLangSystemClass")]
    pub s_java_lang_system_class: crate::unity_engine::globaljavaobjectref::GlobalJavaObjectRef,
    #[static_field]
    #[rename(name = "s_HashCodeMethodID")]
    pub s_hash_code_method_id: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-androidjavaproxy")]
#[::unity2::methods]
impl AndroidJavaProxy {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, java_interface: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        java_interface: crate::unity_engine::androidjavaclass::AndroidJavaClass,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        method_name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::unity_engine::androidjavaobject::AndroidJavaObject;

    #[method(name = "Invoke", args = 2)]
    pub fn invoke_2(
        self,
        method_name: ::unity2::Il2CppString,
        java_args: ::unity2::Array<crate::unity_engine::androidjavaobject::AndroidJavaObject>,
    ) -> crate::unity_engine::androidjavaobject::AndroidJavaObject;

    #[method(name = "GetProxyObject", args = 0)]
    pub fn get_proxy_object(self) -> crate::unity_engine::androidjavaobject::AndroidJavaObject;

    #[method(name = "GetRawProxy", args = 0)]
    pub fn get_raw_proxy(self) -> ::unity2::IntPtr;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-androidjavaproxy")]
impl AndroidJavaProxy {
    pub fn new(java_interface: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaProxy),
                ::core::stringify!(new),
            )
        });
        <Self as IAndroidJavaProxyMethods>::ctor(this, java_interface);
        this
    }

    pub fn new_2(java_interface: crate::unity_engine::androidjavaclass::AndroidJavaClass) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaProxy),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAndroidJavaProxyMethods>::ctor_2(this, java_interface);
        this
    }
}
