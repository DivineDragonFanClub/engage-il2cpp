
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scriptingutility/ScriptingUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ScriptingUtility")]
#[parent(crate::system::object::Object)]
pub struct ScriptingUtility {}

#[cfg(feature = "unity_engine-scriptingutility")]
#[::unity2::methods]
impl ScriptingUtility {
    #[method(name = "IsManagedCodeWorking", args = 0)]
    pub fn is_managed_code_working() -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scriptingutility/ScriptingUtility_TestClass.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ScriptingUtility_TestClass {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ScriptingUtility_TestClass {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ScriptingUtility.TestClass";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptingUtility_TestClass {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
