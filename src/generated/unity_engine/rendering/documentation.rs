
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::documentationinfo::DocumentationInfo;
use crate::unity_engine::rendering::documentationinfo::IDocumentationInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/documentation/Documentation.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "Documentation")]
#[parent(crate::unity_engine::rendering::documentationinfo::DocumentationInfo)]
pub struct Documentation {
    #[static_field]
    #[rename(name = "baseURL")]
    pub base_url: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "subURL")]
    pub sub_url: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "endURL")]
    pub end_url: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "baseURLHDRP")]
    pub base_urlhdrp: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-rendering-documentation")]
#[::unity2::methods]
impl Documentation {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-documentation")]
impl Documentation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Documentation),
                ::core::stringify!(new),
            )
        });
        <Self as IDocumentationMethods>::ctor(this);
        this
    }
}
