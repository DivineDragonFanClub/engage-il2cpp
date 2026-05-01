
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtextdecodata/ProfileCardTextDecoData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTextDecoData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: profilecardtextdecodata :: ProfileCardTextDecoData >)]
pub struct ProfileCardTextDecoData {}

#[cfg(feature = "app-profilecardtextdecodata")]
#[::unity2::methods]
impl ProfileCardTextDecoData {
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

    #[method(name = "GetNameImage", args = 0)]
    pub fn get_name_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommentImage", args = 0)]
    pub fn get_comment_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleLargeImage", args = 0)]
    pub fn get_title_large_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleSmallImage", args = 0)]
    pub fn get_title_small_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnImage", args = 0)]
    pub fn get_column_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetLineImage", args = 0)]
    pub fn get_line_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetUnitNameImage", args = 0)]
    pub fn get_unit_name_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAchievement1Image", args = 0)]
    pub fn get_achievement1_image(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAchievement2Image", args = 0)]
    pub fn get_achievement2_image(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardtextdecodata")]
impl ProfileCardTextDecoData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTextDecoData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTextDecoDataMethods>::ctor(this);
        this
    }
}
