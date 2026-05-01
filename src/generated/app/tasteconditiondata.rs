
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tasteconditiondata/TasteConditionData_ConditionFunc.md")))]
#[::unity2::class(namespace = "App", name = "TasteConditionData.ConditionFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct TasteConditionData_ConditionFunc {}

#[cfg(feature = "app-tasteconditiondata")]
#[::unity2::methods]
impl TasteConditionData_ConditionFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> bool;
}

#[cfg(feature = "app-tasteconditiondata")]
impl TasteConditionData_ConditionFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TasteConditionData_ConditionFunc),
                ::core::stringify!(new),
            )
        });
        <Self as ITasteConditionData_ConditionFuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tasteconditiondata/TasteConditionData.md")))]
#[::unity2::class(namespace = "App", name = "TasteConditionData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: tasteconditiondata :: TasteConditionData >)]
pub struct TasteConditionData {}

#[cfg(feature = "app-tasteconditiondata")]
#[::unity2::methods]
impl TasteConditionData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetConditionFunc", args = 0)]
    pub fn get_condition_func(
        self,
    ) -> crate::app::tasteconditiondata::TasteConditionData_ConditionFunc;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-tasteconditiondata")]
impl TasteConditionData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TasteConditionData),
                ::core::stringify!(new),
            )
        });
        <Self as ITasteConditionDataMethods>::ctor(this);
        this
    }
}
