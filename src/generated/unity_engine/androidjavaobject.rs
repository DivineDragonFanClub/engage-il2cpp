
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjavaobject/AndroidJavaObject.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJavaObject")]
#[parent(crate::system::object::Object)]
pub struct AndroidJavaObject {
    #[static_field]
    #[rename(name = "enableDebugPrints")]
    pub enable_debug_prints: bool,
    #[rename(name = "m_jobject")]
    pub m_jobject: crate::unity_engine::globaljavaobjectref::GlobalJavaObjectRef,
    #[rename(name = "m_jclass")]
    pub m_jclass: crate::unity_engine::globaljavaobjectref::GlobalJavaObjectRef,
}

#[cfg(feature = "unity_engine-androidjavaobject")]
#[::unity2::methods]
impl AndroidJavaObject {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        class_name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetRawObject", args = 0)]
    pub fn get_raw_object(self) -> ::unity2::IntPtr;

    #[method(name = "GetRawClass", args = 0)]
    pub fn get_raw_class(self) -> ::unity2::IntPtr;

    #[method(name = "DebugPrint", args = 1)]
    pub fn debug_print(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "_AndroidJavaObject", args = 2)]
    pub fn android_java_object(
        self,
        class_name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, jobject: ::unity2::IntPtr) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_3(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = "AndroidJavaObjectDeleteLocalRef", args = 1)]
    pub fn android_java_object_delete_local_ref(
        jobject: ::unity2::IntPtr,
    ) -> crate::unity_engine::androidjavaobject::AndroidJavaObject;

    #[method(name = "AndroidJavaClassDeleteLocalRef", args = 1)]
    pub fn android_java_class_delete_local_ref(
        jclass: ::unity2::IntPtr,
    ) -> crate::unity_engine::androidjavaclass::AndroidJavaClass;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-androidjavaobject")]
impl AndroidJavaObject {
    pub fn new(
        class_name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaObject),
                ::core::stringify!(new),
            )
        });
        <Self as IAndroidJavaObjectMethods>::ctor(this, class_name, args);
        this
    }

    pub fn new_2(jobject: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaObject),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAndroidJavaObjectMethods>::ctor_2(this, jobject);
        this
    }

    pub fn new_3() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaObject),
                ::core::stringify!(new_3),
            )
        });
        <Self as IAndroidJavaObjectMethods>::ctor_3(this);
        this
    }
}
