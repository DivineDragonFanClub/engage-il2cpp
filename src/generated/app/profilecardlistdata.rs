
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardlistdata/ProfileCardListData.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardListData")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: profilecardlistdata :: ProfileCardListData >)]
pub struct ProfileCardListData {}

#[cfg(feature = "app-profilecardlistdata")]
#[::unity2::methods]
impl ProfileCardListData {
    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "get_List", args = 0)]
    pub fn get_list(self) -> crate::app::profilelist::ProfileList;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardlistdata")]
impl ProfileCardListData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardListData),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardListDataMethods>::ctor(this);
        this
    }
}
