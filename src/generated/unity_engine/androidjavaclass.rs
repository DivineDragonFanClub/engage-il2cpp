
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::androidjavaobject::AndroidJavaObject;
use crate::unity_engine::androidjavaobject::IAndroidJavaObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjavaclass/AndroidJavaClass.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJavaClass")]
#[parent(crate::unity_engine::androidjavaobject::AndroidJavaObject)]
pub struct AndroidJavaClass {}

#[cfg(feature = "unity_engine-androidjavaclass")]
#[::unity2::methods]
impl AndroidJavaClass {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, class_name: ::unity2::Il2CppString) -> ();

    #[method(name = "_AndroidJavaClass", args = 1)]
    pub fn android_java_class(self, class_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, jclass: ::unity2::IntPtr) -> ();
}

#[cfg(feature = "unity_engine-androidjavaclass")]
impl AndroidJavaClass {
    pub fn new(class_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaClass),
                ::core::stringify!(new),
            )
        });
        <Self as IAndroidJavaClassMethods>::ctor(this, class_name);
        this
    }

    pub fn new_2(jclass: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AndroidJavaClass),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAndroidJavaClassMethods>::ctor_2(this, jclass);
        this
    }
}
