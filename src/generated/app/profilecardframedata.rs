
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardframedata/ProfileCardFrameData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFrameData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: profilecardframedata :: ProfileCardFrameData >)]
pub struct ProfileCardFrameData {}

#[cfg(feature = "app-profilecardframedata")]
#[::unity2::methods]
impl ProfileCardFrameData {
    #[method(name = "get_Id", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Image", args = 0)]
    pub fn get_image(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Image", args = 1)]
    pub fn set_image(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> crate::app::profilecardcondition::ProfileCardCondition;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: crate::app::profilecardcondition::ProfileCardCondition)
        -> ();

    #[method(name = "get_Arg", args = 0)]
    pub fn get_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Arg", args = 1)]
    pub fn set_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardframedata")]
impl ProfileCardFrameData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFrameData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFrameDataMethods>::ctor(this);
        this
    }
}
