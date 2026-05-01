
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/textmeshutility/TextMeshUtility.md")))]
#[::unity2::class(namespace = "App", name = "TextMeshUtility")]
#[parent(crate::system::object::Object)]
pub struct TextMeshUtility {}

#[cfg(feature = "app-textmeshutility")]
#[::unity2::methods]
impl TextMeshUtility {
    #[method(name = "ParseInputText", args = 1)]
    pub fn parse_input_text(root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-textmeshutility")]
impl TextMeshUtility {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextMeshUtility),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMeshUtilityMethods>::ctor(this);
        this
    }
}
