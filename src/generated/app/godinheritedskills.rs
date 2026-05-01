
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godinheritedskills/GodInheritedSkills.md")))]
#[::unity2::class(namespace = "App", name = "GodInheritedSkills")]
#[parent(crate::system::object::Object)]
pub struct GodInheritedSkills {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Sids")]
    pub m_sids: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
}

#[cfg(feature = "app-godinheritedskills")]
#[::unity2::methods]
impl GodInheritedSkills {
    #[method(name = "Add", args = 1)]
    pub fn add(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Has", args = 1)]
    pub fn has(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godinheritedskills")]
impl GodInheritedSkills {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodInheritedSkills),
                ::core::stringify!(new),
            )
        });
        <Self as IGodInheritedSkillsMethods>::ctor(this);
        this
    }
}
