
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecarddefaultcommentdata/ProfileCardDefaultCommentData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardDefaultCommentData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: profilecarddefaultcommentdata :: ProfileCardDefaultCommentData >)]
pub struct ProfileCardDefaultCommentData {}

#[cfg(feature = "app-profilecarddefaultcommentdata")]
#[::unity2::methods]
impl ProfileCardDefaultCommentData {
    #[method(name = "get_Language", args = 0)]
    pub fn get_language(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Language", args = 1)]
    pub fn set_language(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Id1", args = 0)]
    pub fn get_id1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id1", args = 1)]
    pub fn set_id1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Id2", args = 0)]
    pub fn get_id2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id2", args = 1)]
    pub fn set_id2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Id3", args = 0)]
    pub fn get_id3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Id3", args = 1)]
    pub fn set_id3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecarddefaultcommentdata")]
impl ProfileCardDefaultCommentData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardDefaultCommentData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardDefaultCommentDataMethods>::ctor(this);
        this
    }
}
