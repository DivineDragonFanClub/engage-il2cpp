
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkutil/TalkUtil.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkUtil")]
#[parent(crate::system::object::Object)]
pub struct TalkUtil {}

#[cfg(feature = "app-talk3_d-talkutil")]
#[::unity2::methods]
impl TalkUtil {
    #[method(name = "GetChildren", args = 1)]
    pub fn get_children(
        root: crate::unity_engine::gameobject::GameObject,
    ) -> ::unity2::Array<crate::unity_engine::gameobject::GameObject>;

    #[method(name = "PIDToGID", args = 1)]
    pub fn pid_to_gid(pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetTalkerNameByPID", args = 1)]
    pub fn get_talker_name_by_pid(pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkutil")]
impl TalkUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkUtil),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkUtilMethods>::ctor(this);
        this
    }
}
