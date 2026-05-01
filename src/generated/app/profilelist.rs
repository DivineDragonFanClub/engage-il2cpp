
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilelist/ProfileList.md")))]
#[::unity2::class(namespace = "App", name = "ProfileList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: profilecard :: ProfileCard >)]
pub struct ProfileList {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "ProfileCountMax")]
    pub profile_count_max: i32,
}

#[cfg(feature = "app-profilelist")]
#[::unity2::methods]
impl ProfileList {
    #[method(name = "TryGet", args = 1)]
    pub fn try_get(self, index: i32) -> crate::app::profilecard::ProfileCard;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_2(self, owner_id: u64, card: crate::app::profilecard::ProfileCard) -> bool;

    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(self, card: crate::app::profilecard::ProfileCard) -> bool;

    #[method(name = "ForceAdd", args = 1)]
    pub fn force_add(self, card: crate::app::profilecard::ProfileCard) -> bool;

    #[method(name = "IsFull", args = 0)]
    pub fn is_full(self) -> bool;

    #[method(name = "IsOver", args = 0)]
    pub fn is_over(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilelist")]
impl ProfileList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileList),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileListMethods>::ctor(this);
        this
    }
}
